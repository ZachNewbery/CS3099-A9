export const isAuthenticated = () => {
  const token = sessionStorage.getItem("access-token");
  return token !== null;
};

export const getCurrentUser = () => {
  const id = sessionStorage.getItem("username");
  const host = sessionStorage.getItem("host");
  return { id, host };
};