import React from "react";

import { ArcadeButton } from "../../components/ArcadeButton";

export const Pendulum = ({ styles, text }) => {
  return (
    <div id="controller" className="electricity" styles={styles}>
      <p>{text}</p>
      <ArcadeButton onClick={() => console.log("Clicked!")} />
      <p>
        Pendulum: <b>Crush 'em!</b>
      </p>
    </div>
  );
};
