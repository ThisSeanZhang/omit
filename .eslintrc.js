module.exports = {
  root: true,
  env: {
    node: true,
  },
  extends: [
    'plugin:vue/vue3-essential',
    '@vue/airbnb',
    '@vue/typescript/recommended',
  ],
  parserOptions: {
    ecmaVersion: 2020,
  },
  rules: {
    // 'no-console': process.env.NODE_ENV === 'production' ? 'error' : 'off',
    'import/no-extraneous-dependencies': ['error', { devDependencies: true }],
    'no-console': 'off',
    'no-debugger': process.env.NODE_ENV === 'production' ? 'error' : 'off',
    'arrow-parens': [
      2,
      'as-needed',
    ],
    camelcase: 'off',
    'class-methods-use-this': 'off',
    'lines-between-class-members': 'off',
    'max-classes-per-file': 'off',
    'no-shadow': 'off',
    '@typescript-eslint/no-shadow': ['error'],
    // 'class-methods-use-this': ['error', {
    //   exceptMethods: [
    //     // react lifecycle methods, from the airbnb rule
    //     'render',
    //     'getInitialState',
    //     'getDefaultProps',
    //     'getChildContext',
    //     'componentWillMount',
    //     'componentDidMount',
    //     'componentWillReceiveProps',
    //     'shouldComponentUpdate',
    //     'componentWillUpdate',
    //     'componentDidUpdate',
    //     'componentWillUnmount',

    //     // vue lifecycle methods
    //     'beforeCreate',
    //     'created',
    //     'beforeMount',
    //     'mounted',
    //     'beforeUpdate',
    //     'updated',
    //     'activated',
    //     'deactivated',
    //     'beforeDestroy',
    //     'destroyed',
    //     'errorCaptured',
    //   ],
    // }],
  },
};
