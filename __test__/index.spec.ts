import test from 'ava'

import { parse } from '../index'

test('sync function from native code', (t) => {
  t.deepEqual(parse('inmemory:'), {
    scheme: 'inmemory',
    authority: '',
    path: '',
    query: '',
    fragment: '',
  })
})
