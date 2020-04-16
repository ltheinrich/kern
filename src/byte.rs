//! Byte-level operations

/// Split bytes at most n times
pub fn splitn<'a, D: AsRef<[u8]>>(n: usize, data: &'a D, sep: impl AsRef<[u8]>) -> Vec<&'a [u8]> {
    // as ref
    let sep = sep.as_ref();
    let mut data = data.as_ref();

    // scan for seperator position
    let mut buf = Vec::new();
    while let Some(pos) = scan(&data, &sep) {
        // split and add to buf
        let (split, rest) = data.split_at(pos);
        buf.push(split);

        // remove seperator
        data = rest.split_at(sep.len()).1;

        // check if n-length reached
        if buf.len() < n {
            break;
        }
    }

    // add remaining bytes and return
    buf.push(data);
    buf
}

/// Split bytes
pub fn split<'a, D: AsRef<[u8]>>(data: &'a D, sep: impl AsRef<[u8]>) -> Vec<&'a [u8]> {
    // as ref
    let sep = sep.as_ref();
    let mut data = data.as_ref();

    // scan for seperator position
    let mut buf = Vec::new();
    while let Some(pos) = scan(&data, &sep) {
        // split and add to buf
        let (split, rest) = data.split_at(pos);
        buf.push(split);

        // remove seperator
        data = rest.split_at(sep.len()).1;
    }

    // add remaining bytes and return
    buf.push(data);
    buf
}

/// Returns index of first byte in pattern
pub fn scan(data: impl AsRef<[u8]>, pat: impl AsRef<[u8]>) -> Option<usize> {
    // as ref
    let data: &[u8] = data.as_ref();
    let pat: &[u8] = pat.as_ref();

    // checks
    if pat.len() > data.len() {
        return None;
    }

    // iterate through data bytes
    let mut found = 0usize;
    for (i, &d) in data.iter().enumerate() {
        // check if pattern matches
        if d == pat[found] {
            // found another byte
            found += 1;

            // check if found all
            if found == pat.len() {
                // return index of first byte in pattern
                return Some(i + 1 - pat.len());
            }
        } else {
            // pattern interrupted
            found = 0;
        }
    }

    // not found
    None
}
