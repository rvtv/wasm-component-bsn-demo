const validate = function(value, length) {
  return value.length === length;
};

/*
 * The export constant name should be equal to the interface name
 * but transformed from snake case to camel case
 */
export const stringLengthValidation = {
  validate,
};