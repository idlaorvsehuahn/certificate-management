/**
 * Formats an ISO date string into a consistent, timezone-independent UTC display format.
 * Example output: "July 3, 2026 at 01:54 PM"
 */
export const formatUtcDate = (isoString: string): string => {
  const date = new Date(isoString);
  if (isNaN(date.getTime())) return 'N/A';
  
  const months = [
    'January', 'February', 'March', 'April', 'May', 'June',
    'July', 'August', 'September', 'October', 'November', 'December'
  ];
  
  const day = date.getUTCDate();
  const month = months[date.getUTCMonth()];
  const year = date.getUTCFullYear();
  
  let hours = date.getUTCHours();
  const minutes = String(date.getUTCMinutes()).padStart(2, '0');
  const ampm = hours >= 12 ? 'PM' : 'AM';
  hours = hours % 12;
  hours = hours ? hours : 12; // hour '0' becomes '12'
  const formattedHours = String(hours).padStart(2, '0');
  
  return `${month} ${day}, ${year} at ${formattedHours}:${minutes} ${ampm}`;
};

/**
 * Formats an ISO date string into a short YYYY-MM-DD UTC format.
 * Example output: "2026-07-03"
 */
export const formatUtcDateShort = (isoString: string): string => {
  const date = new Date(isoString);
  if (isNaN(date.getTime())) return 'N/A';
  const yyyy = date.getUTCFullYear();
  const mm = String(date.getUTCMonth() + 1).padStart(2, '0');
  const dd = String(date.getUTCDate()).padStart(2, '0');
  return `${yyyy}-${mm}-${dd}`;
};
