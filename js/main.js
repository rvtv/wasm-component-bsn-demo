import { stringValidationRunner as bsnValidation } from './components/bsn-validator/bsn_validator_composed.js';

const validateHandler = () => {
  const bsn = document.getElementById("bsn").value;
  const messagePar = document.getElementById("message");

  try {
    bsnValidation.validate(bsn);
    messagePar.innerText = 'bsn passed all checks';
  } catch (e) {
    messagePar.innerText = `error: ${e.message}`;
  }
};

document.querySelector('input').addEventListener('click', validateHandler);