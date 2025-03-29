import { stringValidationRunner as bsnValidation } from '../../../components/bsn-validator/bsn_validator_composed.js';

const bsn = process.argv[2];

//console.log(bsnValidation.validate(bsn));

try {
  bsnValidation.validate(bsn);
  console.log('bsn passed all checks')
} catch (e) {
  console.log(`error: ${e.message}`);
  process.exit(1);
}
