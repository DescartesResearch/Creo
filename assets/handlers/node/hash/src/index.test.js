import { describe, it } from 'node:test';
import assert from 'node:assert/strict';
import { hashPassword } from './index.js';

describe('hashPassword', () => {
  it('should hash password', async () => {
    const result = await hashPassword('s3cr3t');

    assert.ok(result.hash);
    assert.ok(typeof result.hash === 'string');
  });
});
