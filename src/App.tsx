// src/App.tsx
//
/// 应用根组件

import React from 'react';
import MainLayout from './components/MainLayout/MainLayout';
import './styles/global.less';

function App() {
  return (
    <div className="App">
      <MainLayout />
    </div>
  );
}

export default App;
