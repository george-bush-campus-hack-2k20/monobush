import React from "react";

import { ArcadeButton } from "../../components/ArcadeButton";

export const Spikes = ({ styles, text }) => {
  return (
    <div id="controller" className="electricity" styles={styles}>
    <p>Spikes</p>
      <p>{text}</p>
      <ArcadeButton onClick={() => console.log("Clicked!")} />
      <p>
        <b>Drop 'em!</b>
      </p>
      <ArcadeButton onClick={() => console.log("Clicked!")} />
      <p>
        <b>Skewer 'em!</b>
      </p>
    </div>
  );
};
