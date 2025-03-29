import { validate as stringLength } from 'summit25:validator/string-length-validation@0.0.1';
import { validate as bsnNineProof } from 'summit25:validator/string-validation@0.0.1';

const validate = function(value) {
  if (!stringLength(value, 9)) {
    throw 'bsn should consist of nine digits';
  }

  if (!bsnNineProof(value)) {
    throw 'bsn should be "nine proof"';
  }

  return 'bsn valid';
};

/*
 * The export constant name should be equal to the interface name
 * but transformed from snake case to camel case
 */
export const stringValidationRunner = {
  validate,
};