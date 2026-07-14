# Journal des décisions

> Format ADR léger. Une entrée par décision significative.

---

## Template

```markdown
### DEC-XXX — Titre (YYYY-MM-DD)

**Statut :** proposé | accepté | rejeté | remplacé par DEC-YYY

**Contexte :** …

**Décision :** …

**Conséquences :** …
```

---

## Entrées

### DEC-001 — Documentation canonique dans `docs/` (2026-07-14)

**Statut :** accepté

**Contexte :** Besoin de conserver un contexte riche pour l'agent tout en ayant des références stables (devices, E-APO, audits datés).

**Décision :**
- `agent.md` = hub agent (objectifs, statut, liens)
- `docs/` = référence canonique
- Audits datés immutables dans `docs/audit/`

**Conséquences :** Les mises à jour d'inventaire vont dans `devices.md` et `equalizer-apo.md` ; les snapshots système vont dans de nouveaux fichiers `audit/YYYY-MM-DD.md`.

---

### DEC-002 — Priorité test Soundpad avant VoiceMeeter (2026-07-14)

**Statut :** proposé

**Contexte :** Audit local montre UniteFx déjà installé sur le micro fifine ; VoiceMeeter absent ; chaîne VST lourde (8 plugins).

**Décision proposée :**
1. Tester Soundpad lancé + mesurer latence ressentie.
2. Si latence excessive → alléger VST (retirer 2e Limiter6, réduire plugins).
3. Si besoin duplication / mix complexe → installer VoiceMeeter Banana.

**Conséquences :** Pas d'installation immédiate VoiceMeeter ; PoC Soundpad d'abord.

**Note :** Partiellement remplacé par DEC-003 pour la piste dev custom.

---

### DEC-003 — Fork : duplication VB-Cable vs VST injecteur fin de chaîne (2026-07-14)

**Statut :** proposé

**Contexte :** Deux options identifiées :
1. Dupliquer le micro vers VB-Cable (2e sortie virtuelle).
2. VST custom en **fin** de chaîne E-APO, alimenté par une 2e source (VB-Cable Output ou autre micro) via app compagnon + ring buffer.

**Décision proposée :**
- **Option 2** = piste principale pour **injection soundboard** (objectif #2 et #3).
- **Option 1** = piste secondaire si besoin **duplication** vers OBS / 2e app (objectif #1), via VoiceMeeter ou app WASAPI — pas via VST seul.
- Option 2 peut utiliser VB-Cable comme **entrée** de la soundboard (playback → CABLE Input, capture → app → VST).

**Conséquences :**
- Spec dev : VST `InjectMix-x64.dll` + `SoundboardCompanion.exe` (WASAPI → shared memory).
- Retirer Soundpad UniteFx si Option 2 validée (éviter double injection).
- Option 1 reportée sauf besoin OBS parallèle confirmé.

---

### DEC-004 — Rack / mixer virtuel pour routage global (2026-07-14)

**Statut :** proposé

**Contexte :** Besoin élargi — router **tous** les flux audio (micro, VB-Cable, Behringer, soundboard, casque, OBS) vers n'importe quelle sortie.

**Décision proposée :**
- **Court terme :** VoiceMeeter **Potato** = rack clé en main (5×5), compatible E-APO sur bus B1.
- **Long terme :** rack custom (E3) seulement si UI/intégration Stream Deck spécifique requise.
- Option 2 (VST InjectMix) reste valide si refus de VoiceMeeter, mais ne remplace **pas** un rack complet.

**Conséquences :** DEC-003 et DEC-004 convergent vers VM Potato comme PoC rack ; VST inject = plan B minimaliste.

---

### DEC-005 — MixMixer : app WASAPI Rust (2026-07-14)

**Statut :** accepté

**Contexte :** Refus VoiceMeeter (trop lourd) ; besoin minimaliste mix voix post-E-APO + soundboard (WAV + VB-Cable) → micro virtuel + monitor casque.

**Décision :**
- Implémenter **MixMixer** en Rust (`cpal` / WASAPI)
- Sources : micro fifine (post-E-APO), CABLE Output, WAV hotkeys
- Sorties : CABLE Input (Discord), fifine SC3 (monitor, défaut on)
- Discord micro = CABLE Output ; désactiver Soundpad UniteFx

**Conséquences :**
- Code dans `d:\Audio\mix-mixer\`
- Spec : `docs/dev-mix-mixer.md`
- DEC-002/003/004 partiellement supersédés pour l'injection soundboard
