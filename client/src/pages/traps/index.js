import React from "react";

import "./index.scss"

import { Spikes } from "./Spikes";
import { Pendulum } from "./Pendulum";
import { Electricity } from "./Electricity";

export const Trap = ({ trapData, client, userId }) => {
  const { type, color, text } = trapData;

  const styles = {
    color
  };

  return <Spikes userId={userId} client={client} styles={styles} text={text} />;
  
};
