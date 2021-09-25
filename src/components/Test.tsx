import React from 'react';

const Styles = require("./test.module.scss")

const Test = ({message}) => (
  <article>
    <p className={Styles.test}>
      {message}
    </p> 
  </article>
);

export default Test
