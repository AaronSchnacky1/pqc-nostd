cargo :    Compiling pqc-nostd v0.0.2 (C:\Users\aaron\OneDrive\Desktop\pqc-nostd)
At line:1 char:1
+ cargo test --features 'ml-kem,ml-dsa,fips_140_3' create_complete_kat_ ...
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling pq...ktop\pqc-nostd):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
warning: unused imports: `KyberCiphertext`, `KyberPrivateKey`, `KyberPublicKey`, and `KyberSharedSecret`
  --> src\kat.rs:18:5
   |
18 |     KyberPublicKey, KyberPrivateKey, KyberCiphertext, KyberSharedSecret,
   |     ^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused imports: `DilithiumSignature`, `DilithiumSigningKey`, and `DilithiumVerifyingKey`
  --> src\kat.rs:25:5
   |
25 |     DilithiumVerifyingKey, DilithiumSigningKey, DilithiumSignature, FIPS_CONTEXT,
   |     ^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src\kat.rs:149:51
    |
149 |       let expected_sk: [u8; ML_KEM_1024_SK_BYTES] = [
    |  ______________________--------------------------___^
    | |                      |    |
    | |                      |    help: consider specifying the actual array length: `3232`
    | |                      expected due to this
150 | |     0x66, 0xa8, 0x09, 0xdc, 0x19, 0x09, 0xea, 0x72, 0xc5, 0xc1, 0xe7, 0x89, 0xce, 0xac, 0x29, 0x5e, 
151 | |     0x41, 0x16, 0x72, 0x5c, 0x05, 0x3c, 0x6b, 0x84, 0xeb, 0x4b, 0x14, 0x38, 0x41, 0x1b, 0x32, 0x64, 
152 | |     0xaa, 0xc3, 0xbc, 0xbf, 0xbe, 0x48, 0x1c, 0x2f, 0x93, 0x75, 0xd2, 0x03, 0x6c, 0x16, 0xd5, 0x97, 
...   |
351 | |     0x15, 0xa6, 0xc1, 0xc1, 0x76, 0x67, 0x64, 0x75, 0x38, 0x3d, 0x0e, 0x09, 0x6c, 0x85, 0x6e, 0x63, 
352 | |     ];
    | |_____^ expected an array with a size of 3168, found one with a size of 3232

For more information about this error, try `rustc --explain E0308`.
warning: `pqc-nostd` (lib test) generated 2 warnings
error: could not compile `pqc-nostd` (lib test) due to 1 previous error; 2 warnings emitted
warning: build failed, waiting for other jobs to finish...
warning: `pqc-nostd` (lib) generated 2 warnings (2 duplicates)
error: could not compile `pqc-nostd` (lib) due to 1 previous error; 2 warnings emitted
