import { useEffect, useState } from 'react';

function App() {
  const [status, setStatus] = useState('Tracking input...');
  const [logPath, setLogPath] = useState('');

  useEffect(() => {
    setLogPath('input_log.csv');
  }, []);

  return (
    <div className="container" style={{
      padding: '2rem',
      maxWidth: '800px',
      margin: '0 auto',
      fontFamily: 'system-ui, -apple-system, sans-serif'
    }}>
      <h1 style={{ color: '#2C3E50' }}>Input Tracker</h1>
      <div style={{
        background: '#F8F9FA',
        padding: '1rem',
        borderRadius: '8px',
        marginTop: '1rem'
      }}>
        <p style={{ color: '#2C3E50' }}>
          <strong>Status:</strong> {status}
        </p>
        <p style={{ color: '#2C3E50' }}>
          <strong>Log File:</strong> {logPath}
        </p>
      </div>
      <div style={{
        marginTop: '2rem',
        padding: '1rem',
        background: '#E9ECEF',
        borderRadius: '8px'
      }}>
        <h2 style={{ color: '#2C3E50', fontSize: '1.2rem' }}>About</h2>
        <p style={{ color: '#2C3E50' }}>
          This application tracks mouse movements, clicks, and keyboard input.
          All events are logged to a CSV file with timestamps.
        </p>
      </div>
    </div>
  );
}

export default App;
