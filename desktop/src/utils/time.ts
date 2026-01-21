const WORDS_PER_MINUTE = 150;

/**
 * Estimates the duration of spoken text in seconds.
 * Assumes an average speaking rate of 150 words per minute.
 *
 * @param text The text to estimate duration for
 * @returns Estimated duration in seconds
 */
export const estimateDuration = (text: string): number => {
  if (!text || text.trim().length === 0) {
    return 0;
  }
  const words = text.trim().split(/\s+/).length;
  return (words / WORDS_PER_MINUTE) * 60;
};
