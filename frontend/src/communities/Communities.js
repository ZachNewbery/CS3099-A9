import React, { useContext, useEffect } from "react";
import { useAsync } from "react-async";

import { fetchData, Spinner, Error } from "../helpers";
import { SingleCommunity } from "./SingleCommunity";
import { ListCommunities } from "./ListCommunities";
import { InstanceContext } from "../App";

const fetchCommunities = async ({ instance }) => {
  const url = new URL(`${process.env.REACT_APP_API}/communities`);
  const appendParam = (key, value) => value && url.searchParams.append(key, value);
  appendParam("host", instance);
  return fetchData(url);
};

export const Communities = () => {
  const { instance } = useContext(InstanceContext);

  return <CommunitiesComponent key={instance} instance={instance} />;
};

export const CommunitiesComponent = ({ instance }) => {
  const { data: communities, isLoading, error, reload } = useAsync(fetchCommunities, { instance });

  if (isLoading) return <Spinner />;
  if (error) return <Error message={error} />;

  return (
    <>
      <SingleCommunity communities={communities} refresh={reload} />
      <ListCommunities communities={communities} refresh={reload} />
    </>
  );
};
