import React, { useCallback, useMemo, useState } from 'react';
import {
  Pressable,
  ScrollView,
  StatusBar,
  StyleSheet,
  Text,
  useColorScheme,
  View,
} from 'react-native';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import NativeSlhDsa from 'react-native-slh-dsa/NativeSlhDsa';
type BenchRow = {
  size: number;
  signMs: number;
  verifyMs: number;
  totalMs: number;
};

const installNative = (): boolean => {
  if (!NativeSlhDsa || typeof NativeSlhDsa.installRustCrate !== 'function') {
    return false;
  }
  return Boolean(NativeSlhDsa.installRustCrate());
};

export default function App() {
  const isDarkMode = useColorScheme() === 'dark';
  const [rows, setRows] = useState<BenchRow[]>([]);
  const [status, setStatus] = useState<string>('Idle');
  const [installed, setInstalled] = useState<boolean>(false);

  const textColor = useMemo(
    () => ({ color: isDarkMode ? '#fff' : '#111' }),
    [isDarkMode],
  );

  const handleInstall = useCallback(() => {
    try {
      const ok = installNative();
      setInstalled(ok);
      setStatus(ok ? 'Native module installed' : 'Native module not available');
    } catch (err: any) {
      setStatus(`Install failed: ${String(err)}`);
    }
  }, []);

  const handleRun = useCallback(() => {
    try {
      setStatus('Running bench...');
      const { runBench } = require('./src/bench');
      const result = runBench();
      setRows(result);
      setStatus('Done');
    } catch (err: any) {
      setStatus(`Bench failed: ${String(err)}`);
    }
  }, []);

  return (
    <SafeAreaProvider>
      <StatusBar barStyle={isDarkMode ? 'light-content' : 'dark-content'} />
      <View style={styles.container}>
        <Text style={[styles.title, textColor]}>SLH-DSA Bench (SHAKE-256f)</Text>
        <Text style={[styles.status, textColor]}>{status}</Text>
        <View style={styles.buttonRow}>
          <Pressable style={styles.button} onPress={handleInstall}>
            <Text style={styles.buttonText}>Install JSI</Text>
          </Pressable>
          <Pressable
            style={[styles.button, !installed && styles.buttonDisabled]}
            onPress={handleRun}
            disabled={!installed}
          >
            <Text style={styles.buttonText}>Run Bench</Text>
          </Pressable>
        </View>
        <ScrollView style={styles.output}>
          <Text style={[styles.header, textColor]}>
            size_bytes,sign_ms,verify_ms,total_ms
          </Text>
          {rows.map(row => (
            <Text key={row.size} style={[styles.row, textColor]}>
              {row.size},{row.signMs.toFixed(6)},{row.verifyMs.toFixed(6)},
              {row.totalMs.toFixed(6)}
            </Text>
          ))}
        </ScrollView>
      </View>
    </SafeAreaProvider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    paddingHorizontal: 16,
    paddingTop: 24,
  },
  title: {
    fontSize: 20,
    fontWeight: '600',
    marginBottom: 8,
  },
  status: {
    marginBottom: 12,
  },
  buttonRow: {
    flexDirection: 'row',
    gap: 12,
    marginBottom: 16,
  },
  button: {
    backgroundColor: '#1f2937',
    paddingHorizontal: 16,
    paddingVertical: 10,
    borderRadius: 8,
  },
  buttonDisabled: {
    opacity: 0.5,
  },
  buttonText: {
    color: '#fff',
    fontWeight: '600',
  },
  output: {
    flex: 1,
  },
  header: {
    fontWeight: '600',
    marginBottom: 6,
  },
  row: {
    fontFamily: 'Menlo',
    fontSize: 12,
  },
});
