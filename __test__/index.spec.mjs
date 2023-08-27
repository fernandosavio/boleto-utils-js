import test from 'ava'

import { validate } from '../index.js'

test('validate from native', (t) => {
  const result = validate('00190000090246420601618160730182558620000077352');

  t.is(result.isValid, true);
});


