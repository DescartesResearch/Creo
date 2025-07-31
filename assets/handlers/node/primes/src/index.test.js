import { describe, it } from 'node:test';
import assert from 'node:assert/strict';
import { generateFirstPrimes } from './index.js';

describe('generateFirstPrimes', () => {
  it('should generate first 10 primes', async () => {
    const primes = generateFirstPrimes(10);

    assert.strictEqual(primes.length, 10);
    assert.deepEqual(primes, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
  });
});
