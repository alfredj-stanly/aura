# AURA — Adaptive User Resonance Architecture

> *"Absence is the highest form of presence; AURA listens when users say nothing."*

Probabilistic identity inference at enterprise scale; fast, ethical and reliable.

---

## What AURA Does

When a user signs in via SSO, we capture their name, email and optionally their profile picture. AURA takes these lightweight signals and probabilistically infers:

| Attribute | Output Type | Primary Signals |
|-----------|-------------|-----------------|
| **Gender** | Probability distribution | Name, email, profile image |
| **Age** | Bucketed probability distribution | Name, profile image |
| **Organization** | Deterministic + confidence | Email domain |
| **Region Hint** | Probability distribution | Email TLD, name patterns |
| **Edge Case Flag** | Boolean | Null detection, bot signals, anomalies |

**Key principle**: All outputs are *soft inference only*. No hard labels. No biometric storage. Just distributions, confidence scores and explainability.

## Architecture Overview

```
[ User SSO Sign-in ]
         ↓
[ Identity Enrichment Event ]
         ↓
[ Pre-Silver SCV Tables ]
         ↓
[ AURA Core ]  ←→  [ OpenAI / Bedrock ]
         ↓
[ SCV Enriched Tables ]
         ↓
[ Permutive Activation ]
```

AURA sits between raw identity capture and activation platforms. It **never blocks login** and **never persists raw biometric data**.

## Tech Stack

- **Core Language**: Rust (for deterministic latency; no GC pauses, no memory churn)
- **LLM Integration**: OpenAI API (primary), AWS Bedrock, or multi-modal
- **API Framework**: ntex
- **Runtime**: tokio

## API Endpoints

### `POST /infer` (v0)

The original inference endpoint returning full probability distributions.

**Request:**
```json
{
  "email": "jane.smith@company.com",
  "first_name": "Jane",
  "last_name": "Smith",
  "profile_pic_url": "https://example.com/photo.jpg",
  "provider": "google"
}
```

**Response:**
```json
{
  "gender": {
    "male": 0.08,
    "female": 0.89,
    "other": 0.03
  },
  "age_bucket": {
    "18-24": 0.10,
    "25-34": 0.55,
    "35-44": 0.28,
    "45+": 0.07
  },
  "organization": "company.com",
  "region_hint": "North America",
  "confidence": 0.82,
  "edge_case": false
}
```

### `POST /gaze` (v1)

The newer, more flexible endpoint with single-value outputs and reasoning.

**Request:**
```json
{
  "email": "jane.smith@company.com",
  "name": "Jane Smith",
  "profile_pic_url": "https://example.com/photo.jpg",
  "browsing_history": ["vogue.com/fashion", "gq.com/style"]
}
```

**Response:**
```json
{
  "gender": "female",
  "age_group": "25-34",
  "confidence": "strong",
  "reasoning": {
    "gender": "Name 'Jane' strongly associated with female gender across cultures",
    "age": "Profile image suggests mid-20s to early-30s; browsing patterns consistent with younger professional demographic"
  }
}
```

## Inference Pipeline

### Phase 1: Email Signal Extraction
- Domain parsing → organization resolution
- Country TLD hints (`.co.uk` → UK, `.de` → Germany)
- Username format analysis

### Phase 2: Name Analysis
- First/last name converted to embeddings
- Matched against gender, age, and region clusters
- Cultural name pattern recognition

### Phase 3: Profile Image Inference (Optional)
- **Never stored, never used for identity recognition**
- Infers broad age band and apparent gender probability
- Acts as a *confidence multiplier*, not a primary signal

### Phase 4: Bayesian Fusion
- All upstream signals fused into final distributions
- Confidence score calculated
- Edge cases flagged

## Legal & Privacy

AURA is built with privacy as a first principle:

- ✅ First-party data only
- ✅ Probabilistic inference only (no hard labels)
- ✅ No biometric identification
- ✅ No raw image persistence
- ✅ US-only inference on US brands (legal approved)
- ✅ Full lineage tracking in SCV

All outputs are **advisory signals**—never treated as ground truth.

## Business Impact

AURA directly enables:

- **Cold-start personalization** for new users
- **Higher-value activation cohorts** for advertisers
- **Improved CPM yield** through better targeting
- **Better advertiser trust** in first-party data
- **Reduced reliance** on third-party signals (goodbye, cookies 👋)

### Success Metrics

- % of enriched audience (target: 10% → 20% → 30%+)
- CPM uplift
- CTR uplift
- Cost per 1,000 inferences
- P95 inference latency

## Roadmap

| Phase | Description | Status |
|-------|-------------|--------|
| Phase 0 | Strategic lock, legal sign-off | ✅ Complete |
| Phase 1 | Identity data capture (Google SSO) | ✅ Complete |
| Phase 2 | AURA v0 text inference | ✅ Complete |
| Phase 3 | Profile image inference | 🔄 In Progress |
| Phase 4 | Behavioral calibration (SCV resonance) | 📋 Planned |
| Phase 5 | Economic governor (cost vs revenue) | 📋 Planned |

## Team

**Responsible**: Jez Mundy, Allie Zhang, Utkarsh Srivastava, Aj Stanly  
**Accountable**: Charl Porter, Kiran Suryanarayana, Sudipta  
**Consulted**: Legal, Data Governance  

---

*Internal use only. Proprietary system.*
