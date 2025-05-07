/*
Copyright (c) 2023 Ade M Ramdani <qcynaut@gmail.com>

This software is proprietary and licensed to MyRTS under the terms of the Closed-Source Software License for Freelancers, which is available at https://dictionary.cambridge.org/us/dictionary/english/license.

MyRTS owns all right, title, and interest in and to the software, including all intellectual property rights therein.
MyRTS may use the software for any purpose, including commercial use.
MyRTS may modify the software, but only for their own internal use.
MyRTS may not distribute the software or any modified versions of the software to third parties.
MyRTS may not reverse engineer the software.
MyRTS may not create derivative works from the software.

MyRTS agrees to credit you as the developer of the software in all promotional materials and documentation for the software.

If MyRTS violates any of these terms, their license to use the software will automatically terminate.
*/

/// Base64.
/// Used to encode data.
pub struct Base64;

impl Base64 {
    /// Encode the given data.
    pub fn enc(item: &str) -> String {
        use base64::Engine;
        let eng = base64::engine::general_purpose::STANDARD;
        eng.encode(item.as_bytes())
    }

    /// Decode the given data.
    pub fn dec(b64: &str) -> Option<String> {
        use base64::Engine;
        let eng = base64::engine::general_purpose::STANDARD;
        let decoded = eng.decode(b64.as_bytes());
        match decoded {
            Ok(x) => Some(String::from_utf8_lossy(&x).to_string()),
            Err(_) => None,
        }
    }
}
