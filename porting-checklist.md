# Porting checklist

Remaining structs to port, ordered roughly by dependency. Some file-local helper structs are grouped by subsystem.

## Phase 1: Public API foundations
- [ ] `mmfile_t` (`xdiff-c/xdiff.h`)
- [ ] `mmbuffer_t` (`xdiff-c/xdiff.h`)
- [ ] `xpparam_t` (`xdiff-c/xdiff.h`)
- [ ] `xdemitcb_t` (`xdiff-c/xdiff.h`)
- [ ] `xdemitconf_t` (`xdiff-c/xdiff.h`)
- [ ] `bdiffparam_t` (`xdiff-c/xdiff.h`)

## Phase 2: Public merge config
- [ ] `xmparam_t` (`xdiff-c/xdiff.h`)

## Phase 3: Core diff engine
- [ ] `xdalgoenv_t` (`xdiff-c/xdiffi.h`)
- [ ] `xdpsplit_t` (`xdiff-c/xdiffi.c`)
- [ ] `xdchange_t` (`xdiff-c/xdiffi.h`)

## Phase 4: Preparation / classification
- [ ] `xdlclass_t` (`xdiff-c/xprepare.c`)
- [ ] `xdlclassifier_t` (`xdiff-c/xprepare.c`)

## Phase 5: Emit subsystem
- [ ] `func_line` (`xdiff-c/xemit.c`)

## Phase 6: Patience diff subsystem
- [ ] `hashmap.entry` (`xdiff-c/xpatience.c`)
- [ ] `hashmap` (`xdiff-c/xpatience.c`)

## Phase 7: Histogram diff subsystem
- [ ] `histindex.record` (`xdiff-c/xhistogram.c`)
- [ ] `histindex` (`xdiff-c/xhistogram.c`)
- [ ] `region` (`xdiff-c/xhistogram.c`)

## Phase 8: Compaction / heuristic helpers
- [ ] `split_measurement` (`xdiff-c/xdiffi.c`)
- [ ] `split_score` (`xdiff-c/xdiffi.c`)
- [ ] `xdlgroup` (`xdiff-c/xdiffi.c`)
