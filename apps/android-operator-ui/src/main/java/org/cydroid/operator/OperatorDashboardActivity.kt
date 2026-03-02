package org.cydroid.operator

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.flow.MutableStateFlow
import org.cydroid.aln.NeuromorphicEvent
import org.cydroid.aln.RowLedger

/**
 * Cydroid Operator Dashboard
 * 
 * Provides field operators with real-time visibility into:
 * - Biophysical Safety Status (Fatigue, Stress)
 * - Swarm State (Tempo, Safety Radius)
 * - Eco-Impact Metrics (Eco-Score)
 * - Consent Status (FPS Bound)
 * 
 * Privacy Note: Raw biophysical data is NOT displayed. Only summarized states
 * are shown to comply with soul.guardrail.spec and neurorights constraints.
 */
class OperatorDashboardActivity : ComponentActivity() {

    // State flows updated by ALN event stream
    private val safetyStatus = MutableStateFlow("SAFE")
    private val ecoScore = MutableStateFlow(1.0f)
    private val swarmTempo = MutableStateFlow(0.0f)
    private val consentValid = MutableStateFlow(true)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MaterialTheme {
                Surface(modifier = Modifier.fillMaxSize()) {
                    DashboardScreen(
                        safetyStatus = safetyStatus.collectAsState(),
                        ecoScore = ecoScore.collectAsState(),
                        swarmTempo = swarmTempo.collectAsState(),
                        consentValid = consentValid.collectAsState()
                    )
                }
            }
        }
        // Start ALN event listener (background service)
        startAlnEventListener()
    }

    private fun startAlnEventListener() {
        // Implementation connects to Cydroid ALN stream
        // Updates state flows on new NeuromorphicEvent or RowEntry
    }
}

@Composable
fun DashboardScreen(
    safetyStatus: State<String>,
    ecoScore: State<Float>,
    swarmTempo: State<Float>,
    consentValid: State<Boolean>
) {
    Column(modifier = Modifier.padding(16.dp)) {
        // Header
        Text(text = "Cydroid Operator Console", style = MaterialTheme.typography.headlineMedium)
        Spacer(modifier = Modifier.height(16.dp))

        // Safety Status Card
        Card(colors = CardDefaults.cardColors(
            containerColor = if (safetyStatus.value == "SAFE") MaterialTheme.colorScheme.green 
                             else MaterialTheme.colorScheme.red
        )) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text(text = "Biophysical Safety", style = MaterialTheme.typography.titleMedium)
                Text(text = "Status: ${safetyStatus.value}", style = MaterialTheme.typography.bodyLarge)
                // Note: No raw EEG/EMG values displayed (Soul Guardrail Compliance)
            }
        }
        Spacer(modifier = Modifier.height(8.dp))

        // Eco-Score Card
        Card {
            Column(modifier = Modifier.padding(16.dp)) {
                Text(text = "Ecological Impact", style = MaterialTheme.typography.titleMedium)
                LinearProgressIndicator(
                    progress = ecoScore.value, 
                    modifier = Modifier.fillMaxWidth().height(8.dp)
                )
                Text(text = "Eco-Score: ${ecoScore.value} (Min: 0.86)", style = MaterialTheme.typography.bodySmall)
            }
        }
        Spacer(modifier = Modifier.height(8.dp))

        // Swarm Control
        Card {
            Column(modifier = Modifier.padding(16.dp)) {
                Text(text = "Swarm State", style = MaterialTheme.typography.titleMedium)
                Text(text = "Tempo: ${swarmTempo.value} ops/sec", style = MaterialTheme.typography.bodyLarge)
                if (!consentValid.value) {
                    Text(text = "WARNING: CONSENT EXPIRED", color = MaterialTheme.colorScheme.error)
                }
            }
        }
    }
}
