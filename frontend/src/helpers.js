export const isAuthenticated = () => {
  const token = localStorage.getItem('access-token');
  return token !== null;
}