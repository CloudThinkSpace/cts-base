pub mod rsa;

pub static PUB_KEY: &str = "-----BEGIN RSA PUBLIC KEY-----
MIIBCgKCAQEAt86pJsUXROjfU+WVNMVSUbm1McAtffPmMgUYxbvpAJEfiPHS4PF4
z0mktdHQbqgEVHfq0GeBKthaVgnlEVwilGtfq8ydiVLMF4ln6pbk7ejpXIku5Zej
w5xp73ZLJWDYwGeaOFDehFf+S4Wz2hpyfjSbbknIzxOLG29WTAN6BKB6ab+bYRgg
99kw8Vd1MoFPBf+MnuwouAW4PXr0mdXZMUZziMYuAFdurtIDkCrTu8bEddghwckB
ARyD0gDvuJxOBhObUSM5CG6q5LNivJIT2aN1MvK3ewNE61FZ3t9VfVIgze1bdCgV
tIK5Rz1Ah5lpd2nSTKjpF16/ayifEU9EMQIDAQAB
-----END RSA PUBLIC KEY-----
";

pub static PRIV_KEY: &str = "-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAt86pJsUXROjfU+WVNMVSUbm1McAtffPmMgUYxbvpAJEfiPHS
4PF4z0mktdHQbqgEVHfq0GeBKthaVgnlEVwilGtfq8ydiVLMF4ln6pbk7ejpXIku
5Zejw5xp73ZLJWDYwGeaOFDehFf+S4Wz2hpyfjSbbknIzxOLG29WTAN6BKB6ab+b
YRgg99kw8Vd1MoFPBf+MnuwouAW4PXr0mdXZMUZziMYuAFdurtIDkCrTu8bEddgh
wckBARyD0gDvuJxOBhObUSM5CG6q5LNivJIT2aN1MvK3ewNE61FZ3t9VfVIgze1b
dCgVtIK5Rz1Ah5lpd2nSTKjpF16/ayifEU9EMQIDAQABAoIBAHH0fyxZLub7FVJX
WNzhpPqkDvEsO0gGSWYjgMs+h+NhFdL6UgpByuuAtcw/q2CfbCfOPNc+BHfCsKb8
9XaAz7OCqkrBzb2QhpXlC3cGXCIfr2Y+asPO8qnYtFjQ1x/yXg9Ta+qUQJB+WVsL
JokGsosKJH8I4aV4cVrt+OuhkCmDI52dPYupRmqko9S85IXynNTBUChvBox55lG5
KzWkwPdLTM4+YZQjf/v61AJ6FxVKhpc8UlvRBV0FZTkeP23k0zPnVXqnOTmjQeRY
+pkgv5hzmWv9ctSgPfLLvPxF0coTiAcSLj6mShWAs1QSPIGP6IjIk+h8UlLF+JCs
pWedVRECgYEA4LbWcLwf4WYkkzxpC6JlUwq/LAJEtVZZYT+j2gvn1A7pX/k/EuCN
dC2N1Ht77aPUpQ1IYTKl7sr5VwNfZOcY69F2oqYQf3pposfr2bg8XYw99FHE+RYE
PMyHC/YntXFe2Nog82sHzjyzzcpItaKToxllW6I5ivm5Yg6GAk0L9G0CgYEA0WXW
HM8T64t2FfhrOcrv7o1zKsmVJCksPlIdNgUbIoa5auCaUU/7WW1IMPADVgV8yyOq
OT13U/oZtcEBkprl38WkuzZTIv8OSk2D9Irc+0rwIvJJIJbKGl3ZRQgLjiDfY68O
uX9NoFbh9hzEQEWAS78sDvnsC5cABz9Uaj8qDFUCgYEAxwhKsTl/R0YQOTT1ma9x
7XaEUveDYdYkfmU+vH91C7dVb4M1IYQw5ej/SWdCIf7M9o/oPub8X5/57t723Elw
Ila5rGBIFpEXBW3r8UAkf3sa2M6gFzsv76X8K1UQYL4Ukx/ItbFkDYBohWqYsgBY
ocp5owrxyq1bTIWunlUdtJUCgYBJLkX+G4DiuQKz+vW+ZOYlgNnXAkxVLu5/eUMs
NeydzGfQql5EPKejQUYRlFAWKCTM+dHr30ds2eNSWfmquufLpM9Ffl7fxDPCVATh
2gYQ7Mev7u0PrqNDQ7lDHuSq72Ii/HMw8QS4HKUr1jHgJGoKDu3e7glIo6Um40Ml
Pp37YQKBgQC7WZ0y6jR6XHWAAdNdrkWt+3ZCyVK+lCqRvF5Yus5og+6210tEen+P
xqxaEhtjDPEXt4FRlWPVlQ8j0w7ylHG72Iclg5Uu1gbDrafuy5W32nhORbNp5zct
nZP21nhzmSma4aBJDv6/4T9kYJJ5B1Se8FQqQv7TzUJwjKK4hGG68w==
-----END RSA PRIVATE KEY-----
";

#[derive(Debug)]
pub enum Error {
    PublicKeyCreateError(String),
    LicenceCreateError(String),
    PublicKeyEncryptError(String),
    PrivateCreateError(String),
    PrivateDecryptError(String),
    HexDecodeError(String),
    LicenceExpired(String),
    LicenceInvalid(String)
}