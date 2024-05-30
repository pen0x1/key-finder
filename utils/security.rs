    /// Encrypts data using AES_256_GCM.
    ///
    /// Generates a nonce, encrypts the input data with the provided key, and returns
    /// the nonce and ciphertext concatenated, base64-encoded.
    ///
    /// # Arguments
    /// * `data` - A slice of the data to encrypt.
    /// * `key` - The encryption key as a slice. Must be 32 bytes long (ENCRYPTION_KEY_LEN).
    ///
    /// # Errors
    /// Returns an error if the key length is invalid, if the nonce could not be generated,
    /// or if encryption fails.
    ///
    /// # Returns
    /// A Result containing the base64 encoded nonce and ciphertext on success, or
    /// an error message on failure.
    pub fn encrypt(data: &[u8], key: &[u8]) -> Result<String, &'static str> {
        // Function implementation remains the same
    }