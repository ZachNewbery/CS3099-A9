export const isAuthenticated = () => {
  const token = localStorage.getItem("access-token");
  return token !== null;
};

export const getCurrentUser = () => {
  const id = localStorage.getItem("username");
  const host = localStorage.getItem("host");
  return { id, host };
};