import React from 'react'

export const Trap = ({ trapData }) => {
  const { type, color, text } = trapData;

  const styles = {
    color
  };

  return (
    <p id={type} styles={styles}>
      {text}
    </p>
  );
};
