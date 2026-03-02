# Advanced CyberRank Matrix Kernel for Cydroid/Cybercore-Brain
# Implements Vector-valued CyberRank algebra for Pareto-safe action selection.
# Aligns with soul.guardrail.spec.v1: NEVER quantifies souls, only actions/particles.
# Designed for high-performance edge devices (NPUs, GPUs).

import simd
import math

# =============================================================================
# CONFIGURATION STRUCTS
# =============================================================================

struct CyberRankConfig:
    var safety_weight: Float32
    var legal_weight: Float32
    var biomech_weight: Float32
    var psych_weight: Float32
    var rollback_weight: Float32
    var neu_budget_threshold: Float32  # NEU psych-risk budget minimum

    fn __init__(inout self, s: Float32, l: Float32, b: Float32, p: Float32, r: Float32, neu: Float32):
        self.safety_weight = s
        self.legal_weight = l
        self.biomech_weight = b
        self.psych_weight = p
        self.rollback_weight = r
        self.neu_budget_threshold = neu

# =============================================================================
# RANK VECTOR STRUCT (Aligned with Cybercore-Brain karma.metric.spec)
# =============================================================================

struct RankVector:
    var safety: Float32
    var legal: Float32
    var biomech: Float32
    var psych: Float32
    var rollback: Float32

    fn __init__(inout self, s: Float32, l: Float32, b: Float32, p: Float32, r: Float32):
        self.safety = s
        self.legal = l
        self.biomech = b
        self.psych = p
        self.rollback = r

    # Check if this vector dominates another (Pareto dominance)
    fn dominates(self, other: RankVector) -> Bool:
        var strictly_better = False
        if self.safety < other.safety: return False
        if self.legal < other.legal: return False
        if self.biomech < other.biomech: return False
        if self.psych < other.psych: return False
        if self.rollback < other.rollback: return False
        
        if self.safety > other.safety: strictly_better = True
        if self.legal > other.legal: strictly_better = True
        if self.biomech > other.biomech: strictly_better = True
        if self.psych > other.psych: strictly_better = True
        if self.rollback > other.rollback: strictly_better = True
        
        return strictly_better

    # Compute weighted score (for tie-breaking only, never for soul scoring)
    fn weighted_score(self, config: CyberRankConfig) -> Float32:
        return (self.safety * config.safety_weight +
                self.legal * config.legal_weight +
                self.biomech * config.biomech_weight +
                self.psych * config.psych_weight +
                self.rollback * config.rollback_weight)

# =============================================================================
# NEU BUDGET CHECK (Psych-Risk Enforcement)
# =============================================================================

struct NEUBudget:
    var current_balance: Float32
    var max_balance: Float32
    var exhaustion_threshold: Float32

    fn __init__(inout self, curr: Float32, max: Float32, thresh: Float32):
        self.current_balance = curr
        self.max_balance = max
        self.exhaustion_threshold = thresh

    fn is_safe(self) -> Bool:
        # NEU budget exhaustion triggers deterministic state transitions
        return self.current_balance >= self.exhaustion_threshold

# =============================================================================
# KERNEL: PARETO FRONT SELECTION
# =============================================================================

fn select_t_safe_action(
    candidates: SIMD[RankVector, 4], 
    config: CyberRankConfig,
    neu_budget: NEUBudget
) -> Int32:
    # Selects the safest action via Pareto dominance, gated by NEU budget.
    # Returns index of selected action, or -1 if NEU budget exhausted.
    
    if not neu_budget.is_safe():
        return -1  # NEU exhaustion triggers rollback/denial
    
    var best_idx = 0
    var best_vector = candidates[0]
    
    for i in range(1, 4):
        var current_vector = candidates[i]
        if current_vector.dominates(best_vector):
            best_vector = current_vector
            best_idx = i
        elif not best_vector.dominates(current_vector):
            # Tie-break using weighted score (only if Pareto equal)
            if current_vector.weighted_score(config) > best_vector.weighted_score(config):
                best_vector = current_vector
                best_idx = i
    
    return best_idx

# =============================================================================
# KERNEL: SOUL GUARDRAIL CHECK (Compile-Time Safety)
# =============================================================================

fn verify_soul_guardrail_compliance(action_rank: RankVector) -> Bool:
    # Ensures action does not violate soul.guardrail.spec.v1
    # Specifically checks that 'psych' risk does not exceed safety envelope
    
    # Soul Guardrail Invariant: Psych risk must never outweigh safety
    if action_rank.psych > action_rank.safety:
        return False
    
    # Soul Guardrail Invariant: Rollback capability must exist
    if action_rank.rollback < 0.5:  # Minimum rollback confidence
        return False
        
    return True
