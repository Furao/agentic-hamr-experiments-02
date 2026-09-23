// Application-owned helper; preserved by HAMR regeneration.

/// Ignore empty or oversized requests; never truncate a transmitted frame.
pub(super) fn bounded_payload(frame: &[u8], size: u16) -> Option<&[u8]> {
    if size == 0 {
        None
    } else {
        frame.get(..usize::from(size))
    }
}

#[cfg(test)]
mod tests {
    use super::bounded_payload;

    #[test]
    fn every_u16_size_is_bounded_and_preserves_bytes() {
        let mut frame = [0u8; 1600];
        for (i, byte) in frame.iter_mut().enumerate() {
            *byte = i as u8;
        }
        for size in 0..=u16::MAX {
            let actual = bounded_payload(&frame, size);
            if (1..=1600).contains(&size) {
                assert_eq!(actual, Some(&frame[..usize::from(size)]));
            } else {
                assert_eq!(actual, None, "size {size} must be dropped");
            }
        }
    }

    #[test]
    fn invalid_request_does_not_prevent_later_valid_lanes() {
        let frame = [0x5a; 1600];
        let sizes = [1601, 64, u16::MAX, 1600];
        let accepted: Vec<_> = sizes.into_iter()
            .filter_map(|size| bounded_payload(&frame, size))
            .map(|payload| payload.len())
            .collect();
        assert_eq!(accepted, [64, 1600]);
    }
}
