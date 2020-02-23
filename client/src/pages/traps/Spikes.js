import React from "react";

import { ArcadeButton } from "../../components/ArcadeButton";

const activateTrap = (client, userId) => async () => {
  return client.post("/client/activate_trap", {
    id: userId
  });
};

export const Spikes = ({ userId, client, styles, text }) => {
  return (
    <div id="controller" className="electricity" styles={styles}>
      <p>Spikes</p>
      <p>{text}</p>
      {/*
      <ArcadeButton onClick={() => console.log("Clicked!")} />
      <p>
        <b>Drop 'em!</b>
      </p>
    */}
      <ArcadeButton onClick={activateTrap(client, userId)} />
      <p>
        <b>Skewer 'em!</b>
      </p>
    </div>
  );
};
