#!/usr/bin/env python3
import pathlib
import xml.etree.ElementTree as ET

ROOT = pathlib.Path(__file__).resolve().parents[5]
SPEC = ROOT / "action-requests/CR-01-add-mavlink-firewall/mavlink_spec"
VERIFIED_OUT = pathlib.Path(__file__).resolve().parents[1] / "src/dialect_verified.rs"
# CRC_EXTRA uses the same CRC-16/MCRF4XX byte recurrence as frame validation.
CRC_INITIAL = 0xffff
BYTE_MASK = 0xff
CRC_MASK = 0xffff
BITS_PER_BYTE = 8
CRC_NIBBLE_SHIFT = 4
CRC_POLYNOMIAL_MIX_SHIFT = 3
SIZES = {"double": 8, "uint64_t": 8, "int64_t": 8, "float": 4,
         "uint32_t": 4, "int32_t": 4, "uint16_t": 2, "int16_t": 2,
         "uint8_t": 1, "int8_t": 1, "char": 1}

def accumulate(byte, crc):
    tmp = byte ^ (crc & BYTE_MASK)
    tmp ^= (tmp << CRC_NIBBLE_SHIFT) & BYTE_MASK
    return ((crc >> BITS_PER_BYTE) ^ (tmp << BITS_PER_BYTE)
            ^ (tmp << CRC_POLYNOMIAL_MIX_SHIFT) ^ (tmp >> CRC_NIBBLE_SHIFT)) & CRC_MASK

def metadata(message):
    crc = CRC_INITIAL
    for byte in (message.attrib["name"] + " ").encode(): crc = accumulate(byte, crc)
    fields = []
    extension = False
    minimum = maximum = 0
    for index, field in enumerate(message):
        if field.tag == "extensions":
            extension = True
            continue
        if field.tag != "field": continue
        base = field.attrib["type"].split("[")[0]
        if base == "uint8_t_mavlink_version": base = "uint8_t"
        count = int(field.attrib["type"].split("[")[1][:-1]) if "[" in field.attrib["type"] else 1
        maximum += SIZES[base] * count
        if not extension: minimum += SIZES[base] * count
        if extension: continue
        fields.append((field, SIZES[base], index))
    fields.sort(key=lambda item: -item[1])
    for field, _, _ in fields:
        field_type = field.attrib["type"]
        base = field_type.split("[")[0]
        if base == "uint8_t_mavlink_version": base = "uint8_t"
        for byte in (base + " ").encode(): crc = accumulate(byte, crc)
        for byte in (field.attrib["name"] + " ").encode(): crc = accumulate(byte, crc)
        if "[" in field_type: crc = accumulate(int(field_type.split("[")[1][:-1]), crc)
    return ((crc & BYTE_MASK) ^ (crc >> BITS_PER_BYTE), minimum, maximum)

messages = {}
for filename in ("minimal.xml", "common.xml", "standard.xml", "ardupilotmega.xml"):
    for message in ET.parse(SPEC / filename).getroot().findall(".//message"):
        messages[int(message.attrib["id"])] = metadata(message)

verified = ["// Generated from the bundled MAVLink XML dialects; do not edit manually.",
            "use vstd::prelude::*;", "", "verus! {"]
for qualifier, name in (("pub open spec fn", "metadata_spec"), ("pub fn", "metadata")):
    return_type = "(result: Option<(u8, u16, u16)>)" if name == "metadata" else "Option<(u8, u16, u16)>"
    signature = f"  {qualifier} {name}(message_id: u32) -> {return_type}"
    verified += [signature]
    if name == "metadata": verified += ["    ensures result == metadata_spec(message_id)"]
    verified += ["  {", "    match message_id {"]
    verified += [f"      {message_id} => Some(({crc}, {minimum}, {maximum}))," for message_id, (crc, minimum, maximum) in sorted(messages.items())]
    verified += ["      _ => None,", "    }", "  }", ""]
verified += ["}", ""]
VERIFIED_OUT.write_text("\n".join(verified))
