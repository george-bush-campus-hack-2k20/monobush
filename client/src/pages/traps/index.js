import React from "react";

import "./index.scss"

import { Spikes } from "./Spikes";
import { Pendulum } from "./Pendulum";
import { Electricity } from "./Electricity";

export const Trap = ({ trapData }) => {
  const { type, color, text } = trapData;

  const styles = {
    color
  };

  switch (type) {
    case "spikes":
      return <Spikes styles={styles} text={text} />;
    case "pendulum":
      return <Pendulum styles={styles} text={text} />;
    case "electricity":
      return <Electricity styles={styles} text={text} />;
    default:
      console.err("Out of cheese!");
      return <p>Out of cheese!</p>
  }
};
