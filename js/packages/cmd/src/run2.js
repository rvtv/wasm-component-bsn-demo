// import { stringLengthValidation as stringLength } from '../../../components/string-length-validator/string_length_validator.js';
import { stringValidation as bsnNineProof } from '../../../components/bsn-nineproof-validator-composed/bsn_nineproof_validator_composed.js';

const bsn = process.argv[2];

// if (!stringLength.validate(bsn, 9)) {
//   console.log('bsn should consist of nine digits');
//   process.exit(1);
// }

if (!bsnNineProof.validate(bsn)) {
  console.log('bsn should be "nine proof"');
  process.exit(1);
}

console.log('bsn passed all checks')
