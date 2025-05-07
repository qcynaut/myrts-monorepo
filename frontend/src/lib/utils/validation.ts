/**
 * Validates an email address using regular expressions.
 *
 * @param email - The email address to validate.
 * @returns True if the email address is valid, false otherwise.
 */
export function validateEmail(email: string): boolean {
	// Regular expression to validate email address
	const emailRegex =
		/^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;

	// Check if the email address matches the regular expression
	return emailRegex.test(email);
}

/**
 * Validates the password.
 * @param password - The password to be validated.
 * @returns True if the password is valid, false otherwise.
 */
export function validatePassword(password: string): boolean {
	// Check if the password length is greater than or equal to 8.
	return password.length >= 8;
}
