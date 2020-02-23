import React from "react";

import { ArcadeButton } from "../../components/ArcadeButton";

export const Electricity = ({ styles, text }) => {
  return (
    <div id="controller" className="electricity" styles={styles}>
      <p>{text}</p>
      <ArcadeButton onClick={() => console.log("Clicked!")} />
      <p>
        Laser: <b>Blast 'em!</b>
      </p>
    </div>
  );
};
