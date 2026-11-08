// Not ported yet: the Java @Inject(TAIL) on SynchedEntityData.assignValues (MixinSynchedEntityData.onWhatever).
// Why it's off: No Horizon function mapped for SynchedEntityData.assignValues (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for SynchedEntityData.assignValues: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SynchedEntityData.assignValues -> (not mapped yet)
//   DataValue.id -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinSynchedEntityData.java:30:
// | @Inject(method = "assignValues", at = @At("TAIL"))
// | public void onWhatever(List<DataValue<?>> entries, CallbackInfo ci) {
// |    if (this.entity instanceof Entity syncedEntity) {
// |       List<DataItem<?>> dataEntries = new ArrayList<>();
// |
// |       for (DataValue<?> serializedEntry : entries) {
// |          DataItem<?> entry = this.itemsById[serializedEntry.id()];
// |          this.assignValue(entry, serializedEntry);
// |          dataEntries.add(entry);
// |       }
// |
// |       new DataWatcherUpdatedEvent<>(syncedEntity, dataEntries).post();
// |    }
// | }
