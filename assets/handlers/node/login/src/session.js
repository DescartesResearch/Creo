import uid from 'uid-safe';
import { sessionResponseSchema, sessionSchema } from './schemas.js';

const sessions = new Map();

/**
 *
 * @param userId {string}
 */
export async function createSession(userId) {
  const sessionId = await uid(24);

  const sessionData = await sessionSchema.validateAsync({
    user_id: userId,
    session_id: sessionId,
  });

  sessions.set(sessionId, sessionData);

  return await sessionResponseSchema.validateAsync({
    token: sessionId,
    exp: sessionData.exp,
  });
}
