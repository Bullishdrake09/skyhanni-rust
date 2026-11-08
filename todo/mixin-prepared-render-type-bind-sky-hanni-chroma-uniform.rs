// Not ported yet: the Java @Inject(INVOKE) on PreparedRenderType.drawFromBuffer (MixinPreparedRenderType.bindSkyHanniChromaUniform).
// Why it's off: No Horizon function mapped for PreparedRenderType.drawFromBuffer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for PreparedRenderType.drawFromBuffer: run `match`.
// Note: The call it hooks (RenderPass.setUniform) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   RenderPass.setUniform -> (not mapped yet)
//   PreparedRenderType.drawFromBuffer -> (not mapped yet)
//   PreparedRenderType.pipeline -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinPreparedRenderType.java:17:
// | @Inject(
// |    method = "drawFromBuffer(Lcom/mojang/blaze3d/buffers/GpuBuffer;Lcom/mojang/blaze3d/buffers/GpuBuffer;Lcom/mojang/blaze3d/IndexType;III)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lcom/mojang/blaze3d/systems/RenderPass;setUniform(Ljava/lang/String;Lcom/mojang/blaze3d/buffers/GpuBufferSlice;)V",
// |       shift = Shift.AFTER
// |    )
// | )
// | private void bindSkyHanniChromaUniform(
// |    GpuBuffer vertexBuffer,
// |    GpuBuffer indexBuffer,
// |    IndexType indexType,
// |    int baseVertex,
// |    int firstIndex,
// |    int indexCount,
// |    CallbackInfo ci,
// |    @Local RenderPass renderPass
// | ) {
// |    PreparedRenderType renderType = (PreparedRenderType)this;
// |    GuiRendererHook.INSTANCE.insertChromaSetUniform(renderPass, renderType.pipeline());
// | }
