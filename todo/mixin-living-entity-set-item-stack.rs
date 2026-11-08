// Not ported yet: the Java @Inject(TAIL) on LivingEntity.setItemSlot (MixinLivingEntity.setItemStack).
// Why it's off: No Horizon function mapped for LivingEntity.setItemSlot (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for LivingEntity.setItemSlot: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EquipmentSlot.getId -> (not mapped yet)
//   LivingEntity.die -> horizon_client::mobs::drops::die (suggested, high)
//   LivingEntity.setItemSlot -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinLivingEntity.java:22:
// | @Inject(method = "setItemSlot", at = @At("TAIL"))
// | public void setItemStack(EquipmentSlot equipment, ItemStack itemStack, CallbackInfo ci) {
// |    new EntityEquipmentChangeEvent((Entity)this, equipment.getId(), itemStack).post();
// | }
