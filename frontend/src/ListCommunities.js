import React from "react";
import { Link } from "react-router-dom";

const loadCommunities = async () => {
  let communities = {}
  
  const localCommunities = await fetchData(`${process.env.REACT_APP_API}/communities`);
  communities.local = localCommunities;
  
  const remoteInstances = await fetchData(`${process.env.REACT_APP_API}/get-instances`);
  
  for (const instance of remoteInstances) {
    const communities = await fetchData(`${process.env.REACT_APP_API}/communities?external=${instance}`);
    communities.instance = communities;
  }

  return communities;
};

const Instance = ({ title, communities }) => {
  return (
    <div>
      <h1>{title}</h1>
      <div>
        {communities.map(community => (
          <Link key={community} to={`/communities/${community}`}>{community}</Link>
        ))}
      </div>
    </div>
  )
}

export const ListCommunities = () => {
  const { data: communities } = useAsync(loadCommunities);

  return (
    <div>
      {Object.entries(communities).map(([title, communities]) => <Instance key={title} communities={communities} />)}
    </div>
  )
}