#!/usr/bin/env python3
import pathlib
import xml.etree.ElementTree as ET

ROOT = pathlib.Path(__file__).resolve().parents[5]
SPEC = ROOT / "action-requests/CR-01-add-mavlink-firewall/mavlink_spec"
OUT = pathlib.Path(__file__).resolve().parents[1] / "src/dialect.rs"
SIZES = {"double": 8, "uint64_t": 8, "int64_t": 8, "float": 4,
         "uint32_t": 4, "int32_t": 4, "uint16_t": 2, "int16_t": 2,
         "uint8_t": 1, "int8_t": 1, "char": 1}

def accumulate(byte, crc):
    tmp = byte ^ (crc & 0xff)
    tmp ^= (tmp << 4) & 0xff
    return ((crc >> 8) ^ (tmp << 8) ^ (tmp << 3) ^ (tmp >> 4)) & 0xffff

def metadata(message):
    crc = 0xffff
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
    return ((crc & 0xff) ^ (crc >> 8), minimum, maximum)

messages = {}
for filename in ("minimal.xml", "common.xml", "standard.xml", "ardupilotmega.xml"):
    for message in ET.parse(SPEC / filename).getroot().findall(".//message"):
        messages[int(message.attrib["id"])] = metadata(message)

lines = ["// Generated from the bundled MAVLink XML dialects; do not edit manually.",
         "pub fn crc_extra(message_id: u32) -> Option<(u8, usize, usize)> {", "    match message_id {"]
lines += [f"        {message_id} => Some(({crc}, {minimum}, {maximum}))," for message_id, (crc, minimum, maximum) in sorted(messages.items())]
lines += ["        _ => None,", "    }", "}", ""]
OUT.write_text("\n".join(lines))
