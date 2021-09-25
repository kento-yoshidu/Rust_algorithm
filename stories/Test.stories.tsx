import React from 'react';
import { ComponentStory, ComponentMeta } from '@storybook/react';

import Test from './Test';
import * as HeaderStories from './Header.stories';

export default {
  title: 'Example/Test',
  component: Test,
} as ComponentMeta<typeof Test>;

const Template: ComponentStory<typeof Test> = (args) => <Test {...args} />;

export const Test1 = Template.bind({});
Test1.args = {
  message: "hogehoge"
};

export const LoggedOut = Template.bind({});
LoggedOut.args = {
  ...HeaderStories.LoggedOut.args,
};
