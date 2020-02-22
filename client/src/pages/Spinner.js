import React from "react"

import { CircularProgress } from "@material-ui/core";

export const Spinner = () => (
  <>
    <p>Please wait while we assign you a trap.</p>
    <CircularProgress color="secondary" />
  </>
)
