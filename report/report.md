# Horizon port report: SkyHanni

`skyhanni` 9.3.0 · SkyHanni-9.3.0-mc26.2.jar · 7558 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 16 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 125 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 22 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: at.hannibal2.skyhanni.SkyHanniModLoader
- `modmenu`: at.hannibal2.skyhanni.compat.ConfigModMenuInterop
- `rei_client`: at.hannibal2.skyhanni.compat.SkyHanniReiPlugin
- `skyblockapi:imc/pv-profile`: at.hannibal2.skyhanni.api.SkyBlockAPIEventWrappers::onPvLoad
- `fabric-client-gametest`: at.hannibal2.skyhanni.test.RepoPatternDumpTest

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| ClientTickEvents.END_CLIENT_TICK | `ClientEvents` | hook `tick` | auto | The Fabric API layer (jfabricapi.rs) runs START_CLIENT_TICK, then the level ticks (START/END_LEVEL_TICK, while in a world), then END_CLIENT_TICK, from Horizon's tick hook, in the window's Game only. |
| ClientPlayConnectionEvents.DISCONNECT | `ClientEvents` | hook `tick` | auto | The Fabric API layer runs JOIN on the window's first tick with a player after none, DISCONNECT on the first tick without one after. |
| ClientPlayConnectionEvents.JOIN | `ClientEvents` | hook `tick` | auto | The Fabric API layer runs JOIN on the window's first tick with a player after none, DISCONNECT on the first tick without one after. |
| ClientLifecycleEvents.CLIENT_STOPPING | `ClientEvents` | hook `frame` | auto | CLIENT_STARTED: the Fabric API layer runs it on the window's first frame. CLIENT_STOPPING has no hook in Horizon a46ce68 (c9d474d has native_world_lifecycle). |
| HudElementRegistry.attachElementBefore | `RenderEvents` | hook `hud` | auto |  |
| ClientCommandRegistrationCallback.EVENT | `CommandsRegistry` | hook `command` | auto | The Fabric API layer's command engine runs the Brigadier tree: literals, integer/long/float/double/bool/string arguments with bounds, executes; requires and suggests are left out. |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Inject(HEAD) on ScreenConstructor.fromPacket | `MenuScreensConstructorMixin.skyHanni$openCustomMenu` | HEAD | needs-mapping | No Horizon function known for ScreenConstructor.fromPacket: run `match`. |
| @Inject(HEAD) on AbstractContainerScreen.extractRenderState | `MixinAbstractContainerScreen.renderHead` | HEAD | needs-mapping | No Horizon function known for AbstractContainerScreen.extractRenderState: run `match`. |
| @Inject(TAIL) on AbstractContainerScreen.extractRenderState | `MixinAbstractContainerScreen.renderTail` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for AbstractContainerScreen.extractRenderState: run `match`. |
| @Inject(INVOKE) on AbstractContainerScreen.extractContents at Lnet/minecraft/client/gui/screens/Screen;extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V | `MixinAbstractContainerScreen.renderBackgroundTexture` | AFTER_CALL | needs-mapping | No Horizon function known for AbstractContainerScreen.extractContents: run `match`. The call it hooks (Screen.extractRenderState) isn't mapped to a Rust function yet. |
| @ModifyArg(INVOKE) on AbstractContainerScreen.extractTooltip at Lnet/minecraft/client/gui/GuiGraphicsExtractor;setTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Ljava/util/List;Ljava/util/Optional;IILnet/minecraft/resources/Identifier;)V | `MixinAbstractContainerScreen.renderBackground` | MODIFY_ARG | needs-mapping | No Horizon function known for AbstractContainerScreen.extractTooltip: run `match`. The call it hooks (GuiGraphicsExtractor.setTooltipForNextFrame) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on AbstractContainerScreen.keyPressed | `MixinAbstractContainerScreen.keyPressed` | HEAD | needs-mapping | No Horizon function known for AbstractContainerScreen.keyPressed: run `match`. |
| @Inject(HEAD) on AbstractContainerScreen.mouseClicked | `MixinAbstractContainerScreen.mouseClicked` | HEAD | needs-mapping | No Horizon function known for AbstractContainerScreen.mouseClicked: run `match`. |
| @ModifyArg(INVOKE) on AbstractContainerScreen.extractLabels at Lnet/minecraft/client/gui/GuiGraphicsExtractor;text(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;IIIZ)V | `MixinAbstractContainerScreen.customForegroundTextColor` | MODIFY_ARG | needs-mapping | No Horizon function known for AbstractContainerScreen.extractLabels: run `match`. The call it hooks (GuiGraphicsExtractor.text) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on AbstractContainerScreen.extractSlotHighlightBack at Lnet/minecraft/world/inventory/Slot;isHighlightable()Z | `MixinAbstractContainerScreen.canBeHighlightedBack` | WRAP_OPERATION | needs-mapping | No Horizon function known for AbstractContainerScreen.extractSlotHighlightBack: run `match`. The call it hooks (Slot.isHighlightable) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on AbstractContainerScreen.extractSlotHighlightFront at Lnet/minecraft/world/inventory/Slot;isHighlightable()Z | `MixinAbstractContainerScreen.canBeHighlightedFront` | WRAP_OPERATION | needs-mapping | No Horizon function known for AbstractContainerScreen.extractSlotHighlightFront: run `match`. The call it hooks (Slot.isHighlightable) isn't mapped to a Rust function yet. |
| @ModifyExpressionValue(INVOKE) on AbstractContainerScreen.mouseClicked at Lnet/minecraft/client/player/LocalPlayer;hasInfiniteMaterials()Z | `MixinAbstractContainerScreen.fixMiddleClick` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for AbstractContainerScreen.mouseClicked: run `match`. |
| @Inject(INVOKE) on AbstractContainerScreen.keyPressed at Lnet/minecraft/client/gui/screens/inventory/AbstractContainerScreen;onClose()V | `MixinAbstractContainerScreen.closeWindowPressed` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for AbstractContainerScreen.keyPressed: run `match`. The call it hooks (AbstractContainerScreen.onClose) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on AbstractContainerScreen.extractContents at Lnet/minecraft/client/gui/screens/inventory/AbstractContainerScreen;extractSlotHighlightBack(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V | `MixinAbstractContainerScreen.backgroundDrawn` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for AbstractContainerScreen.extractContents: run `match`. The call it hooks (AbstractContainerScreen.extractSlotHighlightBack) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on AbstractContainerScreen.extractRenderState | `MixinAbstractContainerScreen.preDraw` | HEAD | needs-mapping | No Horizon function known for AbstractContainerScreen.extractRenderState: run `match`. |
| @Inject(TAIL) on AbstractContainerScreen.extractRenderState | `MixinAbstractContainerScreen.postDraw` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for AbstractContainerScreen.extractRenderState: run `match`. |
| @Inject(INVOKE) on AbstractContainerScreen.extractContents at Lnet/minecraft/client/gui/screens/inventory/AbstractContainerScreen;extractSlotHighlightFront(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V | `MixinAbstractContainerScreen.onForegroundDraw` | AFTER_CALL | needs-mapping | No Horizon function known for AbstractContainerScreen.extractContents: run `match`. The call it hooks (AbstractContainerScreen.extractSlotHighlightFront) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on AbstractContainerScreen.extractSlot | `MixinAbstractContainerScreen.onDrawSlot` | HEAD | needs-mapping | No Horizon function known for AbstractContainerScreen.extractSlot: run `match`. |
| @Inject(RETURN) on AbstractContainerScreen.extractSlot | `MixinAbstractContainerScreen.onDrawSlotReturn` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for AbstractContainerScreen.extractSlot: run `match`. |
| @Inject(INVOKE) on AbstractContainerScreen.extractContents at Lnet/minecraft/client/gui/screens/inventory/AbstractContainerScreen;extractSlotHighlightBack(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V | `MixinAbstractContainerScreen.renderTooltipBackgroundTexture` | AFTER_CALL | needs-mapping | No Horizon function known for AbstractContainerScreen.extractContents: run `match`. The call it hooks (AbstractContainerScreen.extractSlotHighlightBack) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on AbstractHurtingProjectile.createParticleTrail | `MixinAbstractHurtingProjectile.onCreateParticleTrail` | HEAD | needs-mapping | No Horizon function known for AbstractHurtingProjectile.createParticleTrail: run `match`. |
| @Inject(HEAD) on AvatarRenderer.isEntityUpsideDown | `MixinAvatarRenderer.shouldFlipUpsideDown` | HEAD | needs-mapping | No Horizon function known for AvatarRenderer.isEntityUpsideDown: run `match`. |
| @WrapOperation(INVOKE) on BaseFireBlock.animateTick at Lnet/minecraft/world/level/Level;addParticle(Lnet/minecraft/core/particles/ParticleOptions;DDDDDD)V | `MixinBaseFireBlock.onAddParticle` | WRAP_OPERATION | needs-mapping | No Horizon function known for BaseFireBlock.animateTick: run `match`. The call it hooks (Level.addParticle) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on Blaze.aiStep at Lnet/minecraft/world/level/Level;addParticle(Lnet/minecraft/core/particles/ParticleOptions;DDDDDD)V | `MixinBlaze.onAddParticle` | WRAP_OPERATION | needs-mapping | No Horizon function known for Blaze.aiStep: run `match`. The call it hooks (Level.addParticle) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on BlockStateModelSet.get | `MixinBlockStateModelSet.getModel` | HEAD | needs-mapping | No Horizon function known for BlockStateModelSet.get: run `match`. |
| @WrapWithCondition(INVOKE) on CapeLayer.submit at Lcom/mojang/blaze3d/vertex/PoseStack;translate(FFF)V | `MixinCapeLayer.wrapChestplateCapeOffset` | WRAP_OPERATION | needs-mapping | Call `original` only when the condition holds. No Horizon function known for CapeLayer.submit: run `match`. The call it hooks (PoseStack.translate) isn't mapped to a Rust function yet. |
| @ModifyArg(INVOKE) on CapeLayer.submit at Lnet/minecraft/client/renderer/SubmitNodeCollector;submitModel(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;IIILnet/minecraft/client/renderer/feature/ModelFeatureRenderer$CrumblingOverlay;)V | `MixinCapeLayer.replaceRenderLayer` | MODIFY_ARG | needs-mapping | No Horizon function known for CapeLayer.submit: run `match`. The call it hooks (SubmitNodeCollector.submitModel) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on ChatComponent.getHeight | `MixinChatComponent.getHeight` | HEAD | needs-mapping | No Horizon function known for ChatComponent.getHeight: run `match`. |
| @Inject(HEAD) on ChatScreen.mouseClicked | `MixinChatScreen.mouseClicked` | HEAD on horizon_client_render::chat::ChatScreen::click | auto |  |
| @Inject(HEAD) on ClientLevel.addEntity | `MixinClientLevel.onAddEntity` | HEAD | needs-mapping | No Horizon function known for ClientLevel.addEntity: run `match`. |
| @Inject(HEAD) on ClientLevel.addDestroyBlockEffect | `MixinClientLevel.onAddBlockBreakParticles` | HEAD | needs-mapping | No Horizon function known for ClientLevel.addDestroyBlockEffect: run `match`. |
| @Inject(HEAD) on ClientLevel.addBreakingBlockEffect | `MixinClientLevel.onAddBlockBreakingParticles` | HEAD | needs-mapping | No Horizon function known for ClientLevel.addBreakingBlockEffect: run `match`. |
| @ModifyExpressionValue(INVOKE) on ClientPacketListener.tick at Lnet/minecraft/client/gui/components/DebugScreenOverlay;showNetworkCharts()Z | `MixinClientPacketListener.shouldShowPacketSizeAndPingCharts` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for ClientPacketListener.tick: run `match`. |
| @Inject(INVOKE) on ClientPacketListener.handleParticleEvent at Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V | `MixinClientPacketListener.postParticleEvent` | AFTER_CALL | needs-mapping | No Horizon function known for ClientPacketListener.handleParticleEvent: run `match`. The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on ClientPacketListener.handleParticleEvent at Lnet/minecraft/client/multiplayer/ClientLevel;addParticle(Lnet/minecraft/core/particles/ParticleOptions;ZZDDDDDD)V | `MixinClientPacketListener.wrapAddParticle` | WRAP_OPERATION | needs-mapping | No Horizon function known for ClientPacketListener.handleParticleEvent: run `match`. The call it hooks (ClientLevel.addParticle) isn't mapped to a Rust function yet. |
| mixin into ClientboundLevelParticlesPacket with no injectors (interfaces or @Shadow only) | `MixinClientboundLevelParticlesPacket` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(INVOKE) on CommandDispatcher.getCompletionSuggestions at Ljava/lang/String;toLowerCase(Ljava/util/Locale;)Ljava/lang/String; | `MixinCommandDispatcher.getCompletionSuggestions` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for CommandDispatcher.getCompletionSuggestions: run `match`. The call it hooks (String.toLowerCase) isn't mapped to a Rust function yet. |
| @ModifyReturnValue(RETURN) on CommandDispatcher.getCompletionSuggestions | `MixinCommandDispatcher.getCompletionSuggestionsWIthExisting` | RETURN | needs-mapping | No Horizon function known for CommandDispatcher.getCompletionSuggestions: run `match`. |
| @Inject(HEAD) on Connection.genericsFtw | `MixinConnection.handlePacket$Inject$HEAD` | HEAD | needs-mapping | No Horizon function known for Connection.genericsFtw: run `match`. |
| @Inject(HEAD) on Connection.send | `MixinConnection.sendPacketNew` | HEAD | needs-mapping | No Horizon function known for Connection.send: run `match`. |
| @ModifyArg(INVOKE) on ContainerScreen.extractBackground at Lnet/minecraft/client/gui/GuiGraphicsExtractor;blit(Lcom/mojang/blaze3d/pipeline/RenderPipeline;Lnet/minecraft/resources/Identifier;IIFFIIII)V | `MixinContainerScreen.getCustomTexture` | MODIFY_ARG | needs-mapping | No Horizon function known for ContainerScreen.extractBackground: run `match`. The call it hooks (GuiGraphicsExtractor.blit) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on ContainerScreen.extractBackground | `MixinContainerScreen.cancelWardrobeBackground` | HEAD | needs-mapping | No Horizon function known for ContainerScreen.extractBackground: run `match`. |
| @WrapWithCondition(INVOKE) on CustomHeadLayer.submit at Lnet/minecraft/client/renderer/blockentity/SkullBlockRenderer;submitSkull(FLcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/model/object/skull/SkullModelBase;Lnet/minecraft/client/renderer/rendertype/RenderType;ILnet/minecraft/client/renderer/feature/ModelFeatureRenderer$CrumblingOverlay;)V | `MixinCustomHeadLayer.onRenderArmor` | WRAP_OPERATION | needs-mapping | Call `original` only when the condition holds. No Horizon function known for CustomHeadLayer.submit: run `match`. The call it hooks (SkullBlockRenderer.submitSkull) isn't mapped to a Rust function yet. |
| @WrapWithCondition(INVOKE) on CustomHeadLayer.submit at Lnet/minecraft/client/renderer/item/ItemStackRenderState;submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;III)V | `MixinCustomHeadLayer.onRenderItemstackOnHead` | WRAP_OPERATION | needs-mapping | Call `original` only when the condition holds. No Horizon function known for CustomHeadLayer.submit: run `match`. |
| @Inject(INVOKE) on Display.tick at Lnet/minecraft/world/entity/Display;updateRenderSubState(ZF)V | `MixinDisplay.onRenderSubStateUpdated` | AFTER_CALL | needs-mapping | No Horizon function known for Display.tick: run `match`. The call it hooks (Display.updateRenderSubState) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on EnderMan.teleport | `MixinEnderMan.onLivingUpdate` | HEAD | needs-mapping | No Horizon function known for EnderMan.teleport: run `match`. |
| @Inject(RETURN) on Entity.getDisplayName | `MixinEntity.getDisplayName` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for Entity.getDisplayName: run `match`. |
| @Inject(HEAD) on EntityRenderDispatcher.submit | `MixinEntityRenderDispatcher.onRenderPre` | HEAD | needs-mapping | No Horizon function known for EntityRenderDispatcher.submit: run `match`. |
| @Inject(RETURN) on EntityRenderDispatcher.submit | `MixinEntityRenderDispatcher.onRenderPost` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for EntityRenderDispatcher.submit: run `match`. |
| @Inject(HEAD) on EntityRenderDispatcher.shouldRender | `MixinEntityRenderDispatcher.shouldRender` | HEAD | needs-mapping | No Horizon function known for EntityRenderDispatcher.shouldRender: run `match`. |
| mixin into EntityRenderState with no injectors (interfaces or @Shadow only) | `MixinEntityRenderState` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on EntityRenderer.submitNameDisplay | `MixinEntityRenderer.onRenderLabelHead` | HEAD | needs-mapping | No Horizon function known for EntityRenderer.submitNameDisplay: run `match`. |
| @Inject(TAIL) on EntityRenderer.submitNameDisplay | `MixinEntityRenderer.onRenderLabelTail` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for EntityRenderer.submitNameDisplay: run `match`. |
| @Inject(TAIL) on EntityRenderer.extractRenderState | `MixinEntityRenderer.setEntity` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for EntityRenderer.extractRenderState: run `match`. |
| @ModifyArg(INVOKE) on EntityRenderer.submitNameDisplay at Lnet/minecraft/client/renderer/SubmitNodeCollector;submitNameTag(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/phys/Vec3;ILnet/minecraft/network/chat/Component;ZILnet/minecraft/client/renderer/state/level/CameraRenderState;)V | `MixinEntityRenderer.modifyRenderLabelIfPresentArgs` | MODIFY_ARG | needs-mapping | No Horizon function known for EntityRenderer.submitNameDisplay: run `match`. The call it hooks (SubmitNodeCollector.submitNameTag) isn't mapped to a Rust function yet. |
| @ModifyArg(INVOKE) on EquipmentLayerRenderer.renderLayers at Lnet/minecraft/client/renderer/OrderedSubmitNodeCollector;submitModel(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;IIILnet/minecraft/client/renderer/texture/TextureAtlasSprite;ILnet/minecraft/client/renderer/feature/ModelFeatureRenderer$CrumblingOverlay;)V | `MixinEquipmentLayerRenderer.replaceVertexConsumer` | MODIFY_ARG | needs-mapping | No Horizon function known for EquipmentLayerRenderer.renderLayers: run `match`. The call it hooks (OrderedSubmitNodeCollector.submitModel) isn't mapped to a Rust function yet. |
| @Inject(RETURN) on FluidStateModelSet.bake | `MixinFluidStateModelSet.bake` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for FluidStateModelSet.bake: run `match`. |
| @ModifyReturnValue(RETURN) on FluidStateModelSet.get | `MixinFluidStateModelSet.replaceLava` | RETURN | needs-mapping | No Horizon function known for FluidStateModelSet.get: run `match`. |
| @ModifyVariable(HEAD) on Font.prepareText | `MixinFont.modifyOrderedText` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for Font.prepareText: run `match`. |
| @ModifyVariable(HEAD) on Font.prepareText | `MixinFont.modifyString` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for Font.prepareText: run `match`. |
| @ModifyVariable(HEAD) on Font.width | `MixinFont.modifyWidthOrderedText` | MODIFY_VARIABLE on `self.native_text_width(text)` | needs-translation | Selects a Rust local by name: pick it in the target function. Horizon has no function to inject into here, only the helper `self.native_text_width(text)`. |
| @ModifyVariable(HEAD) on Font.width | `MixinFont.modifyWidthString` | MODIFY_VARIABLE on `self.native_text_width(text)` | needs-translation | Selects a Rust local by name: pick it in the target function. Horizon has no function to inject into here, only the helper `self.native_text_width(text)`. |
| @ModifyVariable(HEAD) on Font.width | `MixinFont.modifyWidthFormattedText` | MODIFY_VARIABLE on `self.native_text_width(text)` | needs-translation | Selects a Rust local by name: pick it in the target function. Horizon has no function to inject into here, only the helper `self.native_text_width(text)`. |
| @Inject(INVOKE) on GameRenderer.render at Lnet/minecraft/client/gui/render/GuiRenderer;endFrame()V | `MixinGameRenderer262.clearChromaUniforms` | AFTER_CALL on hook `frame` | needs-translation | Hook points are empty functions: use HEAD on the hook instead of AFTER_CALL. The call it hooks (GuiRenderer.endFrame) isn't mapped to a Rust function yet. Target mapping is approximate. |
| @ModifyReturnValue(RETURN) on GlyphRenderTypes.createForColorTexture | `MixinGlyphRenderTypes.ofMethods` | RETURN | needs-mapping | No Horizon function known for GlyphRenderTypes.createForColorTexture: run `match`. |
| @ModifyReturnValue(RETURN) on GlyphRenderTypes.createForIntensityTexture | `MixinGlyphRenderTypes.ofMethods` | RETURN | needs-mapping | No Horizon function known for GlyphRenderTypes.createForIntensityTexture: run `match`. |
| @Inject(HEAD) on GlyphRenderTypes.select | `MixinGlyphRenderTypes.getRenderLayer` | HEAD | needs-mapping | No Horizon function known for GlyphRenderTypes.select: run `match`. |
| @Inject(RETURN) on GuiGraphicsExtractor.item | `MixinGuiGraphicsExtractor.drawItemPost` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for GuiGraphicsExtractor.item: run `match`. |
| @Inject(RETURN) on GuiGraphicsExtractor.itemDecorations | `MixinGuiGraphicsExtractor.drawItemDecorationsPost` | RETURN on horizon_client_render::hud_overlays::item_decorations | auto | Horizon's RETURN runs at every normal exit and passes the return value on. |
| @Inject(INVOKE) on GuiGraphicsExtractor.componentHoverEffect at Ljava/util/Objects;requireNonNull(Ljava/lang/Object;)Ljava/lang/Object; | `MixinGuiGraphicsExtractor.onRenderComponentHoverEffect` | AFTER_CALL | needs-mapping | No Horizon function known for GuiGraphicsExtractor.componentHoverEffect: run `match`. The call it hooks (Objects.requireNonNull) isn't mapped to a Rust function yet. |
| @ModifyArg(INVOKE) on GuiGraphicsExtractor.componentHoverEffect at Lnet/minecraft/client/gui/Font;split(Lnet/minecraft/network/chat/FormattedText;I)Ljava/util/List; | `MixinGuiGraphicsExtractor.replaceWithNewList` | MODIFY_ARG | needs-mapping | No Horizon function known for GuiGraphicsExtractor.componentHoverEffect: run `match`. |
| @ModifyArg(INVOKE) on GuiGraphicsExtractor.text at Lnet/minecraft/client/renderer/state/gui/GuiRenderState;addText(Lnet/minecraft/client/renderer/state/gui/GuiTextRenderState;)V | `MixinGuiGraphicsExtractor.modifyVisualWordsTextState` | MODIFY_ARG | needs-mapping | No Horizon function known for GuiGraphicsExtractor.text: run `match`. The call it hooks (GuiRenderState.addText) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on GuiRenderer.executeDrawRange | `MixinGuiRenderer.computeChromaBufferSlice` | HEAD | needs-mapping | No Horizon function known for GuiRenderer.executeDrawRange: run `match`. |
| @Inject(INVOKE) on GuiRenderer.executeDrawRange at Lcom/mojang/blaze3d/systems/RenderPass;setUniform(Ljava/lang/String;Lcom/mojang/blaze3d/buffers/GpuBufferSlice;)V | `MixinGuiRenderer.insertChromaSetUniform` | AFTER_CALL | needs-mapping | No Horizon function known for GuiRenderer.executeDrawRange: run `match`. The call it hooks (RenderPass.setUniform) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on GuiRenderer.addElementToMesh at Lnet/minecraft/client/renderer/state/gui/GuiElementRenderState;pipeline()Lcom/mojang/blaze3d/pipeline/RenderPipeline; | `MixinGuiRenderer.replacePipeline` | WRAP_OPERATION | needs-mapping | No Horizon function known for GuiRenderer.addElementToMesh: run `match`. The call it hooks (GuiElementRenderState.pipeline) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on GuiRenderer.render | `MixinGuiRenderer.skyhanni$trackFrameNumber` | HEAD | needs-mapping | No Horizon function known for GuiRenderer.render: run `match`. |
| @Inject(HEAD) on GuiRenderer.preparePictureInPicture | `MixinGuiRenderer.skyhanni$preRenderAtlas` | HEAD | needs-mapping | No Horizon function known for GuiRenderer.preparePictureInPicture: run `match`. |
| @Inject(TAIL) on GuiRenderer.preparePictureInPicture | `MixinGuiRenderer.skyhanni$evictUnusedRealtimeSlots` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for GuiRenderer.preparePictureInPicture: run `match`. |
| @Inject(TAIL) on GuiRenderer.preparePictureInPictureState | `MixinGuiRenderer.skyhanni$prepareSkyHanniItems` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for GuiRenderer.preparePictureInPictureState: run `match`. |
| @ModifyVariable(STORE) on Gui.extractRenderState | `MixinGui_GameRenderer.skyhanni$captureGuiGraphicsExtractor` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for Gui.extractRenderState: run `match`. |
| @Inject(INVOKE) on Gui.extractRenderState at Lnet/minecraft/client/gui/Hud;extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V | `MixinGui_GameRenderer.onRenderStartPhase` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for Gui.extractRenderState: run `match`. The call it hooks (Hud.extractRenderState) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on Gui.extractRenderState at Lnet/minecraft/client/gui/Hud;extractSavingIndicator(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V | `MixinGui_GameRenderer.onRenderEndPhase` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for Gui.extractRenderState: run `match`. The call it hooks (Hud.extractSavingIndicator) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on Gui.extractRenderState at Lnet/minecraft/client/gui/Hud;extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V | `MixinGui_GameRenderer.onRenderTail` | AFTER_CALL | needs-mapping | No Horizon function known for Gui.extractRenderState: run `match`. The call it hooks (Hud.extractRenderState) isn't mapped to a Rust function yet. |
| @Inject(FIELD) on Gui.setScreen at Lnet/minecraft/client/gui/Gui;screen:Lnet/minecraft/client/gui/screens/Screen; | `MixinGui_Minecraft.onSetScreen` | – | needs-translation | FIELD, NEW, JUMP and CONSTANT points have no horizon-mixins counterpart yet. |
| @Inject(HEAD) on Hud.displayScoreboardSidebar | `MixinHud.renderScoreboard` | HEAD | needs-mapping | No Horizon function known for Hud.displayScoreboardSidebar: run `match`. |
| @Inject(HEAD) on Hud.extractItemHotbar | `MixinHud.renderHotbar` | HEAD | needs-mapping | No Horizon function known for Hud.extractItemHotbar: run `match`. |
| @Inject(TAIL) on Hud.extractItemHotbar | `MixinHud.renderHotbarTail` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for Hud.extractItemHotbar: run `match`. |
| @Inject(INVOKE) on Hud.extractTabList at Lnet/minecraft/client/gui/components/PlayerTabOverlay;extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;ILnet/minecraft/world/scores/Scoreboard;Lnet/minecraft/world/scores/Objective;)V | `MixinHud.renderPlayerList` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for Hud.extractTabList: run `match`. The call it hooks (PlayerTabOverlay.extractRenderState) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on Hud.extractHotbarAndDecorations at Lnet/minecraft/client/gui/contextualbar/ContextualBar;extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V | `MixinHud.renderExperienceBar` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for Hud.extractHotbarAndDecorations: run `match`. The call it hooks (ContextualBar.extractBackground) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on Hud.extractHotbarAndDecorations at Lnet/minecraft/client/gui/contextualbar/ContextualBar;extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V | `MixinHud.renderExperienceBarTail` | AFTER_CALL | needs-mapping | No Horizon function known for Hud.extractHotbarAndDecorations: run `match`. The call it hooks (ContextualBar.extractBackground) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on Hud.extractHotbarAndDecorations at Lnet/minecraft/client/gui/contextualbar/ContextualBar;extractExperienceLevel(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/gui/Font;I)V | `MixinHud.renderExperienceLevelHead` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for Hud.extractHotbarAndDecorations: run `match`. The call it hooks (ContextualBar.extractExperienceLevel) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on Hud.extractHotbarAndDecorations at Lnet/minecraft/client/gui/contextualbar/ContextualBar;extractExperienceLevel(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/gui/Font;I)V | `MixinHud.renderExperienceLevelTail` | AFTER_CALL | needs-mapping | No Horizon function known for Hud.extractHotbarAndDecorations: run `match`. The call it hooks (ContextualBar.extractExperienceLevel) isn't mapped to a Rust function yet. |
| @ModifyArg(INVOKE) on Hud.displayScoreboardSidebar at Lnet/minecraft/client/gui/GuiGraphicsExtractor;text(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;IIIZ)V | `MixinHud.modifyScoreboardLine` | MODIFY_ARG | needs-mapping | No Horizon function known for Hud.displayScoreboardSidebar: run `match`. The call it hooks (GuiGraphicsExtractor.text) isn't mapped to a Rust function yet. |
| @ModifyArg(INVOKE) on Hud.extractChat at Lnet/minecraft/client/gui/components/ChatComponent;extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/gui/Font;IIILnet/minecraft/client/gui/components/ChatComponent$DisplayMode;Z)V | `MixinHud.modifyRenderText` | MODIFY_ARG | needs-mapping | No Horizon function known for Hud.extractChat: run `match`. The call it hooks (ChatComponent.extractRenderState) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on Hud.extractSelectedItemName | `MixinHud.renderSelectedItemNamePre` | HEAD | needs-mapping | No Horizon function known for Hud.extractSelectedItemName: run `match`. |
| @Inject(TAIL) on Hud.extractSelectedItemName | `MixinHud.renderSelectedItemNamePost` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for Hud.extractSelectedItemName: run `match`. |
| @Inject(HEAD) on Hud.extractOverlayMessage | `MixinHud.renderOverlayMessagePre` | HEAD | needs-mapping | No Horizon function known for Hud.extractOverlayMessage: run `match`. |
| @Inject(RETURN) on Hud.extractOverlayMessage | `MixinHud.renderOverlayMessagePost` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for Hud.extractOverlayMessage: run `match`. |
| @Inject(HEAD) on HumanoidArmorLayer.renderArmorPiece | `MixinHumanoidArmorLayer.onRenderArmor` | HEAD | needs-mapping | No Horizon function known for HumanoidArmorLayer.renderArmorPiece: run `match`. |
| mixin into Inventory with no injectors (interfaces or @Shadow only) | `MixinInventory` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @ModifyArg(INVOKE) on ItemFeatureRenderer.prepareOutlineSubmit at Lcom/mojang/blaze3d/vertex/QuadInstance;setColor(I)V | `MixinItemFeatureRenderer.modifyAlpha` | MODIFY_ARG | needs-mapping | No Horizon function known for ItemFeatureRenderer.prepareOutlineSubmit: run `match`. The call it hooks (QuadInstance.setColor) isn't mapped to a Rust function yet. |
| @ModifyExpressionValue(INVOKE) on ItemFeatureRenderer.prepareOutlineSubmit at Lnet/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo;itemRenderType()Lnet/minecraft/client/renderer/rendertype/RenderType; | `MixinItemFeatureRenderer.modifyRenderLayer` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for ItemFeatureRenderer.prepareOutlineSubmit: run `match`. |
| @Inject(HEAD) on ItemStack.isEmpty | `MixinItemStack.skyhanni$handleNullHolder` | HEAD | needs-mapping | No Horizon function known for ItemStack.isEmpty: run `match`. |
| @Inject(INVOKE) on ItemStack.addToTooltip at Lnet/minecraft/world/item/component/TooltipProvider;addToTooltip(Lnet/minecraft/world/item/Item$TooltipContext;Ljava/util/function/Consumer;Lnet/minecraft/world/item/TooltipFlag;Lnet/minecraft/core/component/DataComponentGetter;)V | `MixinItemStack.blockVanillaEnchants` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for ItemStack.addToTooltip: run `match`. The call it hooks (TooltipProvider.addToTooltip) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on KeyMapping.isDown | `MixinKeyMapping.noIsKeyDown` | HEAD on `self.native_mapping(name) for vanilla keys; track key hook presses for the mod's own` | needs-translation | Horizon has no function to inject into here, only the helper `self.native_mapping(name) for vanilla keys; track key hook presses for the mod's own`. Target mapping is approximate. |
| @Inject(HEAD) on KeyMapping.consumeClick | `MixinKeyMapping.noIsPressed` | HEAD on `key == self.native_mod_key(..) && pressed, in the key hook` | needs-translation | Horizon has no function to inject into here, only the helper `key == self.native_mod_key(..) && pressed, in the key hook`. |
| @Inject(HEAD) on KeyboardHandler.keyPress | `MixinKeyboardHandler.onKey` | HEAD on hook `key` | auto |  |
| @Inject(HEAD) on KeyboardHandler.charTyped | `MixinKeyboardHandler.onChar` | HEAD | needs-mapping | No Horizon function known for KeyboardHandler.charTyped: run `match`. |
| @Inject(HEAD) on LevelExtractor.extractVisibleEntities | `MixinLevelExtractor.resetRealGlowing` | HEAD | needs-mapping | No Horizon function known for LevelExtractor.extractVisibleEntities: run `match`. |
| @Inject(HEAD) on LivingEntity.die | `MixinLivingEntity.onDeath` | HEAD on horizon_client::mobs::drops::die | auto |  |
| @Inject(TAIL) on LivingEntity.setItemSlot | `MixinLivingEntity.setItemStack` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for LivingEntity.setItemSlot: run `match`. |
| @Inject(TAIL) on LivingEntityRenderer.extractRenderState | `MixinLivingEntityRenderer.updateRenderState` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for LivingEntityRenderer.extractRenderState: run `match`. |
| @ModifyArg(INVOKE) on LivingEntityRenderer.submit at Lnet/minecraft/client/renderer/SubmitNodeCollector;submitModel(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;IIILnet/minecraft/client/renderer/texture/TextureAtlasSprite;ILnet/minecraft/client/renderer/feature/ModelFeatureRenderer$CrumblingOverlay;)V | `MixinLivingEntityRenderer.modifyRenderAlpha` | MODIFY_ARG | needs-mapping | No Horizon function known for LivingEntityRenderer.submit: run `match`. The call it hooks (SubmitNodeCollector.submitModel) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on LivingEntityRenderer.getRenderType | `MixinLivingEntityRenderer.getRenderState` | HEAD | needs-mapping | No Horizon function known for LivingEntityRenderer.getRenderType: run `match`. |
| @Inject(HEAD) on Minecraft.startUseItem | `MixinMinecraft.handleRightClickMouse` | HEAD on horizon_client::first_person::Clicks::start_use_item | auto |  |
| @Inject(HEAD) on Minecraft.startAttack | `MixinMinecraft.handleLeftClickMouse` | HEAD on horizon_client::first_person::Clicks::start_attack | auto |  |
| @ModifyVariable(HEAD) on Minecraft.continueAttack | `MixinMinecraft.handleBlockClick` | MODIFY_VARIABLE on horizon_client::first_person::Clicks::continue_attack | auto | Selects a Rust local by name: pick it in the target function. |
| @ModifyArg(INVOKE) on Model.renderToBuffer at Lnet/minecraft/client/model/geom/ModelPart;render(Lcom/mojang/blaze3d/vertex/PoseStack;Lcom/mojang/blaze3d/vertex/VertexConsumer;III)V | `MixinModel.modifyRenderAlpha` | MODIFY_ARG | needs-mapping | No Horizon function known for Model.renderToBuffer: run `match`. The call it hooks (ModelPart.render) isn't mapped to a Rust function yet. |
| @Inject(RETURN) on MouseHandler.onMove | `MixinMouseHandler.onMouseButton` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for MouseHandler.onMove: run `match`. |
| @Inject(HEAD) on MouseHandler.onScroll | `MixinMouseHandler.onScroll` | HEAD on hook `wheel` | auto |  |
| @Inject(HEAD) on MouseHandler.onButton | `MixinMouseHandler.onMouseButton` | HEAD | needs-mapping | No Horizon function known for MouseHandler.onButton: run `match`. |
| @WrapOperation(INVOKE) on MouseHandler.turnPlayer at Lnet/minecraft/client/player/LocalPlayer;turn(DD)V | `MixinMouseHandler.modifyMouseSensitivity` | WRAP_OPERATION | needs-mapping | No Horizon function known for MouseHandler.turnPlayer: run `match`. |
| mixin into MutableComponent with no injectors (interfaces or @Shadow only) | `MixinMutableComponent` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on ParticleEngine.createParticle | `MixinParticleEngine.onCreateParticle` | HEAD on horizon_client_render::particles::ParticleEngine::create | auto |  |
| @Inject(RETURN) on Player.getDisplayName | `MixinPlayer.getDisplayName` | RETURN on horizon_client::lan::hub::display_name | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. Horizon splits Player.getDisplayName across 5 functions: pick the piece this hooks, then remove the others from `pieces`. |
| @Inject(INVOKE) on PreparedRenderType.drawFromBuffer at Lcom/mojang/blaze3d/systems/RenderPass;setUniform(Ljava/lang/String;Lcom/mojang/blaze3d/buffers/GpuBufferSlice;)V | `MixinPreparedRenderType.bindSkyHanniChromaUniform` | AFTER_CALL | needs-mapping | No Horizon function known for PreparedRenderType.drawFromBuffer: run `match`. The call it hooks (RenderPass.setUniform) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on PreparedTextBuilder.visit at Lnet/minecraft/client/gui/Font$GlyphVisitor;acceptGlyph(Lnet/minecraft/client/gui/font/TextRenderable$Styled;)V | `MixinPreparedTextBuilder.checkIfGlyphIsChroma` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for PreparedTextBuilder.visit: run `match`. The call it hooks (GlyphVisitor.acceptGlyph) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on PreparedTextBuilder.accept at Lnet/minecraft/network/chat/Style;getColor()Lnet/minecraft/network/chat/TextColor; | `MixinPreparedTextBuilder.wrapGetColor` | WRAP_OPERATION | needs-mapping | No Horizon function known for PreparedTextBuilder.accept: run `match`. The call it hooks (Style.getColor) isn't mapped to a Rust function yet. |
| @ModifyArg(INVOKE) on PreparedTextBuilder.accept at Lnet/minecraft/client/gui/font/glyphs/BakedGlyph;createGlyph(FFIILnet/minecraft/network/chat/Style;FF)Lnet/minecraft/client/gui/font/TextRenderable$Styled; | `MixinPreparedTextBuilder.forceChromaIfNecessary` | MODIFY_ARG | needs-mapping | No Horizon function known for PreparedTextBuilder.accept: run `match`. The call it hooks (BakedGlyph.createGlyph) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on Screen.extractRenderStateWithTooltipAndSubtitles at Lnet/minecraft/client/gui/screens/Screen;extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V | `MixinScreen.wrapExtractBackground` | WRAP_OPERATION | needs-mapping | No Horizon function known for Screen.extractRenderStateWithTooltipAndSubtitles: run `match`. |
| @Inject(HEAD) on ScreenEffectRenderer.submitFire | `MixinScreenEffectRenderer.renderFire` | HEAD | needs-mapping | No Horizon function known for ScreenEffectRenderer.submitFire: run `match`. |
| @Inject(HEAD) on ScreenEffectRenderer.submitWater | `MixinScreenEffectRenderer.renderWater` | HEAD | needs-mapping | No Horizon function known for ScreenEffectRenderer.submitWater: run `match`. |
| @Inject(HEAD) on ScreenEffectRenderer.submitBlockSprite | `MixinScreenEffectRenderer.renderBlock` | HEAD | needs-mapping | No Horizon function known for ScreenEffectRenderer.submitBlockSprite: run `match`. |
| @ModifyExpressionValue(INVOKE) on SimpleContainer.getItem at Lnet/minecraft/core/NonNullList;get(I)Ljava/lang/Object; | `MixinSimpleContainer.getStack` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for SimpleContainer.getItem: run `match`. |
| @Inject(INVOKE) on SoundEngine.play at Lnet/minecraft/client/resources/sounds/SoundInstance;getVolume()F | `MixinSoundEngine.handleSound` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for SoundEngine.play: run `match`. The call it hooks (SoundInstance.getVolume) isn't mapped to a Rust function yet. |
| @ModifyExpressionValue(INVOKE) on SoundEngine.play at Lnet/minecraft/client/sounds/SoundEngine;calculateVolume(FLnet/minecraft/sounds/SoundSource;)F | `MixinSoundEngine.overrideSkyHanniSoundVolume` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for SoundEngine.play: run `match`. |
| @ModifyReturnValue(RETURN) on SoundEngine.calculateVolume | `MixinSoundEngine.overrideSkyHanniSoundVolumeOnRefresh` | RETURN | needs-mapping | No Horizon function known for SoundEngine.calculateVolume: run `match`. |
| @Inject(HEAD) on SplashManager.getSplash | `MixinSplashManager.addSkyhanniSplash` | HEAD | needs-mapping | No Horizon function known for SplashManager.getSplash: run `match`. |
| @Inject(HEAD) on StringDecomposer.iterateFormatted | `MixinStringDecomposer.skyhanni$resetColorState` | HEAD | needs-mapping | No Horizon function known for StringDecomposer.iterateFormatted: run `match`. |
| @WrapOperation(INVOKE) on StringDecomposer.iterateFormatted at Lnet/minecraft/ChatFormatting;getByCode(C)Lnet/minecraft/ChatFormatting; | `MixinStringDecomposer.skyhanni$onGetByCode` | WRAP_OPERATION | needs-mapping | No Horizon function known for StringDecomposer.iterateFormatted: run `match`. The call it hooks (ChatFormatting.getByCode) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on StringDecomposer.iterateFormatted at Lnet/minecraft/network/chat/Style;applyLegacyFormat(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/Style; | `MixinStringDecomposer.skyhanni$onApplyLegacyFormat` | WRAP_OPERATION | needs-mapping | No Horizon function known for StringDecomposer.iterateFormatted: run `match`. The call it hooks (Style.applyLegacyFormat) isn't mapped to a Rust function yet. |
| @ModifyArg(INVOKE) on StringDecomposer.iterateFormatted at Lnet/minecraft/util/StringDecomposer;feedChar(Lnet/minecraft/network/chat/Style;Lnet/minecraft/util/FormattedCharSink;IC)Z | `MixinStringDecomposer.skyhanni$applyActiveHexColor` | MODIFY_ARG | needs-mapping | No Horizon function known for StringDecomposer.iterateFormatted: run `match`. The call it hooks (StringDecomposer.feedChar) isn't mapped to a Rust function yet. |
| @ModifyVariable(INVOKE_ASSIGN) on StringDecomposer.iterateFormatted at Lnet/minecraft/ChatFormatting;getByCode(C)Lnet/minecraft/ChatFormatting; | `MixinStringDecomposer.onColorCodeCheck` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for StringDecomposer.iterateFormatted: run `match`. |
| @ModifyVariable(HEAD) on StringSplitter.splitLines | `MixinStringSplitter.modifyStringVisitable` | MODIFY_VARIABLE on horizon_client_render::font::Font::split_lines | auto | Selects a Rust local by name: pick it in the target function. |
| @Inject(TAIL) on SynchedEntityData.assignValues | `MixinSynchedEntityData.onWhatever` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for SynchedEntityData.assignValues: run `match`. |
| @Inject(TAIL) on TextColor.equals | `MixinTextColor.chromaEqualityFix` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for TextColor.equals: run `match`. |
| @Inject(HEAD) on Callback.onRemove | `MixinTransientEntitySectionManager$Callback.remove` | HEAD | needs-mapping | No Horizon function known for Callback.onRemove: run `match`. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends ContainerScreen | `CustomWardrobeEditScreen` | needs-translation | No mapping for this superclass yet. |
| extends ItemStack | `DeferredItemStack` | needs-translation | No mapping for this superclass yet. |
| extends PlayerTeam | `FakePlayer$getTeam$1` | needs-translation | No mapping for this superclass yet. |
| extends ClientMannequin | `FakePlayer` | needs-translation | No mapping for this superclass yet. |
| extends SimpleSoundInstance | `SkyHanniSoundInstance` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `SkyHanniBaseScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends AbstractTexture | `SkyHanniAbstractItemTexture` | needs-translation | No mapping for this superclass yet. |
| extends PictureInPictureRenderer | `SkyHanniPipCoordinatorRenderer` | needs-translation | No mapping for this superclass yet. |
| extends Vec3 | `SnappedVec3` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `MoulConfigScreenComponent` | needs-translation | No general screen hook; simple option pages can use the settings hook. |

## Access wideners

| File | In | Status | Note |
|---|---|---|---|
| access widener (opens private game fields) | `skyhanni.classtweaker` | needs-translation | Native helpers already see the game's fields, once each field is mapped. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 114 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

59 of 1615 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `com/mojang/blaze3d/platform/InputConstants$Key#getDisplayName` | horizon_client_core::key_display::display_name (suggested) |
| `com/mojang/blaze3d/platform/Window#calculateScale` | horizon_client_input::window::calculate_scale (suggested) |
| `net/minecraft/client/KeyMapping#consumeClick` | `key == self.native_mod_key(..) && pressed, in the key hook` |
| `net/minecraft/client/KeyMapping#isDown` | `self.native_mapping(name) for vanilla keys; track key hook presses for the mod's own` |
| `net/minecraft/client/KeyboardHandler#keyPress` | hook `key` |
| `net/minecraft/client/KeyboardHandler#setClipboard` | horizon_client::game::debug_keys::Game<'_>::set_clipboard (suggested) |
| `net/minecraft/client/Minecraft#continueAttack` | horizon_client::first_person::Clicks::continue_attack (suggested) |
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/Minecraft#startAttack` | horizon_client::first_person::Clicks::start_attack (suggested) |
| `net/minecraft/client/Minecraft#startUseItem` | horizon_client::first_person::Clicks::start_use_item (suggested) |
| `net/minecraft/client/MouseHandler#onScroll` | hook `wheel` |
| `net/minecraft/client/OptionInstance#set` | horizon_client_render::options_file::OptionsFile::set (suggested) |
| `net/minecraft/client/StringSplitter#splitLines` | horizon_client_render::font::Font::split_lines (suggested) |
| `net/minecraft/client/gui/Font#plainSubstrByWidth` | horizon_client_render::font::Font::fit (suggested) |
| `net/minecraft/client/gui/Font#split` | horizon_client_render::font::Font::split_formatted (suggested) |
| `net/minecraft/client/gui/Font#width` | `self.native_text_width(text)` |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blitSprite` | horizon_client_render::ui::blit_sprite (suggested) |
| `net/minecraft/client/gui/GuiGraphicsExtractor#itemDecorations` | horizon_client_render::hud_overlays::item_decorations (suggested) |
| `net/minecraft/client/gui/components/ComponentRenderUtils#wrapComponents` | horizon_client_render::font::Font::wrap_components (suggested) |
| `net/minecraft/client/gui/components/PlayerTabOverlay#getNameForDisplay` | horizon_client_render::tab_list::name_for_display (suggested) |
| `net/minecraft/client/gui/screens/ChatScreen#mouseClicked` | horizon_client_render::chat::ChatScreen::click (suggested) |
| `net/minecraft/client/gui/screens/Screen#extractBackground` | horizon_client::game::screens::Screen::blurs_background (suggested) |
| `net/minecraft/client/multiplayer/ClientPacketListener#sendChat` | horizon_client::game::remote::Game<'_>::send_chat (suggested) |
| `net/minecraft/client/particle/ParticleEngine#createParticle` | horizon_client_render::particles::ParticleEngine::create (suggested) |
| `net/minecraft/client/player/LocalPlayer#isOnFire` | horizon_client::mobs::Mob::is_on_fire (suggested) |
| `net/minecraft/client/player/LocalPlayer#isShiftKeyDown` | `self.native_mapping("key.sneak").is_some_and(|(held, _)| held)` |
| `net/minecraft/client/player/LocalPlayer#onGround` | horizon_client::game::minecarts::Minecart::on_ground (suggested) |
| `net/minecraft/client/player/LocalPlayer#turn` | horizon_client_input::mouse::Rotation::turn (suggested) |
| `net/minecraft/client/renderer/GameRenderer#render` | hook `frame` |
| `net/minecraft/client/renderer/item/ItemStackRenderState#submit` | horizon_client::item_render::ItemRenderState::submit (suggested) |
| `net/minecraft/core/Direction#fromYRot` | horizon_world::level::place::from_y_rot (suggested) |
| `net/minecraft/core/registries/BuiltInRegistries#ITEM` | content |
| `net/minecraft/network/chat/MutableComponent#append` | horizon_client::game::dedicated::append (suggested) |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/network/chat/Style#applyTo` | horizon_client_core::text::Style::inherit (suggested) |
| `net/minecraft/util/ARGB#colorFromFloat` | horizon_client_render::hud_layout::color_from_float (suggested) |
| `net/minecraft/util/ARGB#opaque` | horizon_client_render::level_loading::opaque (suggested) |
| `net/minecraft/util/ARGB#white` | horizon_client_render::hud_layout::white (suggested) |
| `net/minecraft/world/entity/Entity#getX` | `self.native_player()?.x` |
| `net/minecraft/world/entity/Entity#getXRot` | `self.native_player()?.pitch` |
| `net/minecraft/world/entity/Entity#getY` | `self.native_player()?.y` |
| `net/minecraft/world/entity/Entity#getYRot` | `self.native_player()?.yaw` |
| `net/minecraft/world/entity/Entity#getZ` | `self.native_player()?.z` |
| `net/minecraft/world/entity/Entity#isInvisible` | horizon_client::mobs::Mob::is_invisible (suggested) |
| `net/minecraft/world/entity/EntityType#create` | horizon_client::mobs::finalize::create (suggested) |
| `net/minecraft/world/entity/LivingEntity#die` | horizon_client::mobs::drops::die (suggested) |
| `net/minecraft/world/entity/LivingEntity#distanceTo` | horizon_client::lan::waypoints::Player::distance_to (suggested) |
| `net/minecraft/world/entity/LivingEntity#hasEffect` | horizon_client::mobs::Seen::has_effect (suggested) |
| `net/minecraft/world/entity/LivingEntity#isInvisible` | horizon_client::mobs::Mob::is_invisible (suggested) |
| `net/minecraft/world/entity/animal/panda/Panda#getVariant` | horizon_client::mobs::panda::variant (suggested) |
| `net/minecraft/world/entity/player/Player#getDisplayName` | horizon_client::lan::hub::display_name (suggested) |
| `net/minecraft/world/entity/player/PlayerSkin#cape` | horizon_client_render::skins::cape (suggested) |
| `net/minecraft/world/item/ItemStack#getCount` | horizon_world::loot::LootStack::get_count (suggested) |
| `net/minecraft/world/item/ItemStack#hasFoil` | horizon_items::item_data::ItemStack::has_foil (suggested) |
| `net/minecraft/world/item/ItemStackTemplate#create` | horizon_client::crafting::Template::create (suggested) |
| `net/minecraft/world/level/block/Block#withPropertiesOf` | horizon_world::worldgen::structure::processor::with_properties_of (suggested) |
| `net/minecraft/world/phys/AABB#intersects` | horizon_core::geometry::Aabb::intersects_corners (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/GpuFormat#D32_FLOAT` | field | 1 |
| `com/mojang/blaze3d/GpuFormat#RG32_FLOAT` | field | 1 |
| `com/mojang/blaze3d/GpuFormat#RGB32_FLOAT` | field | 1 |
| `com/mojang/blaze3d/GpuFormat#RGBA32_FLOAT` | field | 1 |
| `com/mojang/blaze3d/GpuFormat#RGBA8_UNORM` | field | 3 |
| `com/mojang/blaze3d/PrimitiveTopology#LINES` | field | 1 |
| `com/mojang/blaze3d/PrimitiveTopology#QUADS` | field | 1 |
| `com/mojang/blaze3d/PrimitiveTopology#TRIANGLES` | field | 1 |
| `com/mojang/blaze3d/PrimitiveTopology#TRIANGLE_FAN` | field | 1 |
| `com/mojang/blaze3d/PrimitiveTopology#TRIANGLE_STRIP` | field | 1 |
| `com/mojang/blaze3d/ProjectionType#ORTHOGRAPHIC` | field | 2 |
| `com/mojang/blaze3d/buffers/Std140Builder#intoBuffer` | method | 1 |
| `com/mojang/blaze3d/buffers/Std140Builder#putFloat` | method | 1 |
| `com/mojang/blaze3d/buffers/Std140Builder#putInt` | method | 1 |
| `com/mojang/blaze3d/buffers/Std140SizeCalculator#<init>` | method | 1 |
| `com/mojang/blaze3d/buffers/Std140SizeCalculator#get` | method | 1 |
| `com/mojang/blaze3d/buffers/Std140SizeCalculator#putFloat` | method | 1 |
| `com/mojang/blaze3d/buffers/Std140SizeCalculator#putInt` | method | 1 |
| `com/mojang/blaze3d/pipeline/BindGroupLayout#builder` | method | 1 |
| `com/mojang/blaze3d/pipeline/BindGroupLayout$Builder#build` | method | 1 |
| `com/mojang/blaze3d/pipeline/BindGroupLayout$Builder#withSampler` | method | 1 |
| `com/mojang/blaze3d/pipeline/BindGroupLayout$Builder#withUniform` | method | 1 |
| `com/mojang/blaze3d/pipeline/BlendFunction#TRANSLUCENT` | field | 1 |
| `com/mojang/blaze3d/pipeline/ColorTargetState#<init>` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderPipeline#builder` | method | 2 |
| `com/mojang/blaze3d/pipeline/RenderPipeline$Builder#build` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderPipeline$Builder#buildSnippet` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderPipeline$Builder#withBindGroupLayout` | method | 2 |
| `com/mojang/blaze3d/pipeline/RenderPipeline$Builder#withColorTargetState` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderPipeline$Builder#withCull` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderPipeline$Builder#withDepthStencilState` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderPipeline$Builder#withFragmentShader` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderPipeline$Builder#withLocation` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderPipeline$Builder#withPrimitiveTopology` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderPipeline$Builder#withVertexBinding` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderPipeline$Builder#withVertexShader` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderTarget#getColorTextureView` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderTarget#getDepthTextureView` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderTarget#useDepth` | field | 1 |
| `com/mojang/blaze3d/platform/ClipboardManager#<init>` | method | 1 |
| `com/mojang/blaze3d/platform/ClipboardManager#getClipboard` | method | 1 |
| `com/mojang/blaze3d/platform/ClipboardManager#setClipboard` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants#getKey` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants#isKeyDown` | method | 2 |
| `com/mojang/blaze3d/platform/InputConstants$Key#getValue` | method | 5 |
| `com/mojang/blaze3d/platform/Lighting#setupFor` | method | 2 |
| `com/mojang/blaze3d/platform/Lighting$Entry#ITEMS_3D` | field | 2 |
| `com/mojang/blaze3d/platform/Lighting$Entry#ITEMS_FLAT` | field | 1 |
| `com/mojang/blaze3d/platform/NativeImage#<init>` | method | 1 |
| `com/mojang/blaze3d/platform/NativeImage#setPixel` | method | 2 |
| `com/mojang/blaze3d/platform/NativeImage$Format#RGBA` | field | 1 |
| `com/mojang/blaze3d/platform/TextureUtil#writeAsPNG` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getGuiScale` | method | 5 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledHeight` | method | 5 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledWidth` | method | 4 |
| `com/mojang/blaze3d/platform/Window#getHeight` | method | 2 |
| `com/mojang/blaze3d/platform/Window#getScreenHeight` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getScreenWidth` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getWidth` | method | 3 |
| `com/mojang/blaze3d/platform/Window#handle` | method | 2 |
| `com/mojang/blaze3d/shaders/UniformType#UNIFORM_BUFFER` | field | 1 |
| `com/mojang/blaze3d/systems/CommandEncoder#clearColorAndDepthTextures` | method | 3 |
| `com/mojang/blaze3d/systems/CommandEncoder#createRenderPass` | method | 1 |
| `com/mojang/blaze3d/systems/DeviceInfo#limits` | method | 1 |
| `com/mojang/blaze3d/systems/DeviceLimits#maxTextureSizeForFormat` | method | 1 |
| `com/mojang/blaze3d/systems/GpuDevice#createCommandEncoder` | method | 4 |
| `com/mojang/blaze3d/systems/GpuDevice#createTexture` | method | 1 |
| `com/mojang/blaze3d/systems/GpuDevice#createTextureView` | method | 1 |
| `com/mojang/blaze3d/systems/GpuDevice#getDeviceInfo` | method | 1 |
| `com/mojang/blaze3d/systems/RenderPass#drawIndexed` | method | 1 |
| `com/mojang/blaze3d/systems/RenderPass#enableScissor` | method | 1 |
| `com/mojang/blaze3d/systems/RenderPass#setUniform` | method | 3 |
| `com/mojang/blaze3d/systems/RenderSystem#assertOnRenderThread` | method | 3 |
| `com/mojang/blaze3d/systems/RenderSystem#disableScissorForRenderTypeDraws` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#enableScissorForRenderTypeDraws` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getDevice` | method | 4 |
| `com/mojang/blaze3d/systems/RenderSystem#getSamplerCache` | method | 3 |
| `com/mojang/blaze3d/systems/RenderSystem#getScissorStateForRenderTypeDraws` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#isOnRenderThread` | method | 2 |
| `com/mojang/blaze3d/systems/RenderSystem#outputColorTextureOverride` | field | 2 |
| `com/mojang/blaze3d/systems/RenderSystem#outputDepthTextureOverride` | field | 2 |
| `com/mojang/blaze3d/systems/RenderSystem#setProjectionMatrix` | method | 2 |
| `com/mojang/blaze3d/systems/SamplerCache#getRepeat` | method | 3 |
| `com/mojang/blaze3d/systems/ScissorState#enabled` | method | 1 |
| `com/mojang/blaze3d/systems/ScissorState#height` | method | 1 |
| `com/mojang/blaze3d/systems/ScissorState#width` | method | 1 |
| `com/mojang/blaze3d/systems/ScissorState#x` | method | 1 |
| `com/mojang/blaze3d/systems/ScissorState#y` | method | 1 |
| `com/mojang/blaze3d/textures/FilterMode#LINEAR` | field | 1 |
| `com/mojang/blaze3d/textures/FilterMode#NEAREST` | field | 3 |
| `com/mojang/blaze3d/textures/GpuTexture#close` | method | 1 |
| `com/mojang/blaze3d/textures/GpuTextureView#close` | method | 1 |
| `com/mojang/blaze3d/vertex/BufferBuilder#addVertex` | method | 5 |
| `com/mojang/blaze3d/vertex/BufferBuilder#format` | field | 1 |
| `com/mojang/blaze3d/vertex/BufferBuilder#setColor` | method | 3 |
| `com/mojang/blaze3d/vertex/BufferBuilder#setUv` | method | 1 |
| `com/mojang/blaze3d/vertex/BufferBuilder#vertexPointer` | field | 1 |
| `com/mojang/blaze3d/vertex/DefaultVertexFormat#POSITION_COLOR` | field | 1 |
| `com/mojang/blaze3d/vertex/DefaultVertexFormat#POSITION_COLOR_NORMAL_LINE_WIDTH` | field | 1 |
| `com/mojang/blaze3d/vertex/DefaultVertexFormat#POSITION_TEX_COLOR` | field | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#<init>` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#last` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#mulPose` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack#popPose` | method | 10 |
| `com/mojang/blaze3d/vertex/PoseStack#pushPose` | method | 10 |
| `com/mojang/blaze3d/vertex/PoseStack#scale` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack#translate` | method | 8 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#pose` | method | 2 |
| `com/mojang/blaze3d/vertex/QuadInstance#setColor` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#addVertex` | method | 7 |
| `com/mojang/blaze3d/vertex/VertexConsumer#addVertexWith2DPose` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setColor` | method | 8 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setLineWidth` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setNormal` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexFormat#builder` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexFormat#getElement` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexFormat$Builder#addAttribute` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexFormat$Builder#build` | method | 1 |
| `com/mojang/blaze3d/vertex/VertexFormatElement#offset` | method | 1 |
| `com/mojang/brigadier/CommandDispatcher#getCompletionSuggestions` | method | 2 |
| `java/lang/String#toLowerCase` | method | 1 |
| `java/util/Objects#requireNonNull` | method | 1 |
| `net/minecraft/ChatFormatting#AQUA` | field | 14 |
| `net/minecraft/ChatFormatting#BLUE` | field | 8 |
| `net/minecraft/ChatFormatting#BOLD` | field | 4 |
| `net/minecraft/ChatFormatting#DARK_AQUA` | field | 2 |
| `net/minecraft/ChatFormatting#DARK_GRAY` | field | 5 |
| `net/minecraft/ChatFormatting#DARK_GREEN` | field | 2 |
| `net/minecraft/ChatFormatting#DARK_PURPLE` | field | 8 |
| `net/minecraft/ChatFormatting#DARK_RED` | field | 1 |
| `net/minecraft/ChatFormatting#GOLD` | field | 9 |
| `net/minecraft/ChatFormatting#GRAY` | field | 17 |
| `net/minecraft/ChatFormatting#GREEN` | field | 19 |
| `net/minecraft/ChatFormatting#ITALIC` | field | 2 |
| `net/minecraft/ChatFormatting#LIGHT_PURPLE` | field | 9 |
| `net/minecraft/ChatFormatting#OBFUSCATED` | field | 2 |
| `net/minecraft/ChatFormatting#RED` | field | 15 |
| `net/minecraft/ChatFormatting#RESET` | field | 2 |
| `net/minecraft/ChatFormatting#STRIKETHROUGH` | field | 2 |
| `net/minecraft/ChatFormatting#UNDERLINE` | field | 2 |
| `net/minecraft/ChatFormatting#WHITE` | field | 12 |
| `net/minecraft/ChatFormatting#YELLOW` | field | 14 |
| `net/minecraft/ChatFormatting#getByCode` | method | 1 |
| `net/minecraft/ChatFormatting#ordinal` | method | 2 |
| `net/minecraft/ChatFormatting#toString` | method | 7 |
| `net/minecraft/ChatFormatting#values` | method | 4 |
| `net/minecraft/CrashReport#<init>` | method | 12 |
| `net/minecraft/SharedConstants#getCurrentVersion` | method | 1 |
| `net/minecraft/WorldVersion#name` | method | 1 |
| `net/minecraft/client/Camera#getCullFrustum` | method | 1 |
| `net/minecraft/client/Camera#getFluidInCamera` | method | 1 |
| `net/minecraft/client/Camera#position` | field | 1 |
| `net/minecraft/client/CameraType#isFirstPerson` | method | 1 |
| `net/minecraft/client/CameraType#isMirrored` | method | 1 |
| `net/minecraft/client/DeltaTracker$Timer#getGameTimeDeltaPartialTick` | method | 2 |
| `net/minecraft/client/KeyMapping#key` | field | 5 |
| `net/minecraft/client/KeyMapping#matches` | method | 2 |
| `net/minecraft/client/KeyMapping#releaseAll` | method | 1 |
| `net/minecraft/client/KeyMapping#setDown` | method | 1 |
| `net/minecraft/client/KeyboardHandler#charTyped` | method | 1 |
| `net/minecraft/client/KeyboardHandler#getClipboard` | method | 1 |
| `net/minecraft/client/Minecraft#debugEntries` | field | 1 |
| `net/minecraft/client/Minecraft#delayCrash` | method | 12 |
| `net/minecraft/client/Minecraft#deltaTracker` | field | 2 |
| `net/minecraft/client/Minecraft#execute` | method | 1 |
| `net/minecraft/client/Minecraft#gameMode` | field | 1 |
| `net/minecraft/client/Minecraft#gameRenderer` | field | 7 |
| `net/minecraft/client/Minecraft#getConnection` | method | 2 |
| `net/minecraft/client/Minecraft#getDebugOverlay` | method | 1 |
| `net/minecraft/client/Minecraft#getEntityRenderDispatcher` | method | 1 |
| `net/minecraft/client/Minecraft#getItemModelResolver` | method | 1 |
| `net/minecraft/client/Minecraft#getResourceManager` | method | 2 |
| `net/minecraft/client/Minecraft#getSoundManager` | method | 1 |
| `net/minecraft/client/Minecraft#getTextureManager` | method | 6 |
| `net/minecraft/client/Minecraft#getUser` | method | 1 |
| `net/minecraft/client/Minecraft#getWindow` | method | 12 |
| `net/minecraft/client/Minecraft#gui` | field | 2 |
| `net/minecraft/client/Minecraft#hitResult` | field | 1 |
| `net/minecraft/client/Minecraft#isEnforceUnicode` | method | 2 |
| `net/minecraft/client/Minecraft#isSameThread` | method | 1 |
| `net/minecraft/client/Minecraft#isWindowActive` | method | 1 |
| `net/minecraft/client/Minecraft#keyboardHandler` | field | 1 |
| `net/minecraft/client/Minecraft#level` | field | 4 |
| `net/minecraft/client/Minecraft#levelExtractor` | field | 1 |
| `net/minecraft/client/Minecraft#mouseHandler` | field | 2 |
| `net/minecraft/client/Minecraft#options` | field | 15 |
| `net/minecraft/client/Minecraft#playerSkinRenderCache` | method | 1 |
| `net/minecraft/client/Minecraft#schedule` | method | 1 |
| `net/minecraft/client/MouseHandler#getScaledXPos` | method | 1 |
| `net/minecraft/client/MouseHandler#getScaledYPos` | method | 1 |
| `net/minecraft/client/MouseHandler#onButton` | method | 1 |
| `net/minecraft/client/MouseHandler#onMove` | method | 1 |
| `net/minecraft/client/MouseHandler#turnPlayer` | method | 1 |
| `net/minecraft/client/MouseHandler#xpos` | method | 1 |
| `net/minecraft/client/MouseHandler#ypos` | method | 1 |
| `net/minecraft/client/OptionInstance#get` | method | 5 |
| `net/minecraft/client/Options#advancedItemTooltips` | field | 1 |
| `net/minecraft/client/Options#chatHeightFocused` | method | 1 |
| `net/minecraft/client/Options#discreteMouseScroll` | method | 1 |
| `net/minecraft/client/Options#getCameraType` | method | 1 |
| `net/minecraft/client/Options#guiScale` | method | 3 |
| `net/minecraft/client/Options#keyAttack` | field | 2 |
| `net/minecraft/client/Options#keyDown` | field | 3 |
| `net/minecraft/client/Options#keyFullscreen` | field | 1 |
| `net/minecraft/client/Options#keyInventory` | field | 3 |
| `net/minecraft/client/Options#keyJump` | field | 2 |
| `net/minecraft/client/Options#keyLeft` | field | 3 |
| `net/minecraft/client/Options#keyPlayerList` | field | 1 |
| `net/minecraft/client/Options#keyRight` | field | 3 |
| `net/minecraft/client/Options#keyScreenshot` | field | 1 |
| `net/minecraft/client/Options#keyShift` | field | 2 |
| `net/minecraft/client/Options#keyUp` | field | 3 |
| `net/minecraft/client/Options#keyUse` | field | 1 |
| `net/minecraft/client/Options#mouseWheelSensitivity` | method | 1 |
| `net/minecraft/client/ToggleKeyMapping#needsToggle` | field | 2 |
| `net/minecraft/client/User#getName` | method | 1 |
| `net/minecraft/client/User#getProfileId` | method | 1 |
| `net/minecraft/client/entity/ClientMannequin#<init>` | method | 1 |
| `net/minecraft/client/gui/ActiveTextCollector#findElementUnderCursor` | method | 1 |
| `net/minecraft/client/gui/ActiveTextCollector$Parameters#<init>` | method | 1 |
| `net/minecraft/client/gui/ActiveTextCollector$Parameters#pose` | method | 1 |
| `net/minecraft/client/gui/ActiveTextCollector$Parameters#scissor` | method | 1 |
| `net/minecraft/client/gui/Font#getSplitter` | method | 1 |
| `net/minecraft/client/gui/Font#lineHeight` | field | 2 |
| `net/minecraft/client/gui/Font#prepareText` | method | 2 |
| `net/minecraft/client/gui/Font$DisplayMode#POLYGON_OFFSET` | field | 1 |
| `net/minecraft/client/gui/Font$DisplayMode#SEE_THROUGH` | field | 1 |
| `net/minecraft/client/gui/Font$GlyphVisitor#acceptGlyph` | method | 1 |
| `net/minecraft/client/gui/Font$PreparedTextBuilder#accept` | method | 2 |
| `net/minecraft/client/gui/Font$PreparedTextBuilder#visit` | method | 1 |
| `net/minecraft/client/gui/Gui#extractRenderState` | method | 4 |
| `net/minecraft/client/gui/Gui#hud` | field | 2 |
| `net/minecraft/client/gui/Gui#screen` | method | 1 |
| `net/minecraft/client/gui/Gui#setScreen` | method | 3 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#<init>` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blit` | method | 3 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#componentHoverEffect` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#disableScissor` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#enableScissor` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fill` | method | 3 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fillGradient` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#guiHeight` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#guiRenderState` | field | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#guiWidth` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#hashCode` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#innerBlit` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#innerFill` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#item` | method | 3 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#pose` | method | 5 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#scissorStack` | field | 3 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#setTooltipForNextFrame` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#text` | method | 8 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#tooltip` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor$ScissorStack#peek` | method | 2 |
| `net/minecraft/client/gui/GuiGraphicsExtractor$ScissorStack#stack` | field | 1 |
| `net/minecraft/client/gui/Hud#displayScoreboardSidebar` | method | 2 |
| `net/minecraft/client/gui/Hud#extractChat` | method | 1 |
| `net/minecraft/client/gui/Hud#extractHotbarAndDecorations` | method | 4 |
| `net/minecraft/client/gui/Hud#extractItemHotbar` | method | 2 |
| `net/minecraft/client/gui/Hud#extractOverlayMessage` | method | 2 |
| `net/minecraft/client/gui/Hud#extractRenderState` | method | 2 |
| `net/minecraft/client/gui/Hud#extractSavingIndicator` | method | 1 |
| `net/minecraft/client/gui/Hud#extractSelectedItemName` | method | 2 |
| `net/minecraft/client/gui/Hud#extractTabList` | method | 1 |
| `net/minecraft/client/gui/Hud#getBossOverlay` | method | 1 |
| `net/minecraft/client/gui/Hud#getChat` | method | 8 |
| `net/minecraft/client/gui/Hud#getGuiTicks` | method | 3 |
| `net/minecraft/client/gui/Hud#getTabList` | method | 2 |
| `net/minecraft/client/gui/Hud#isHidden` | method | 1 |
| `net/minecraft/client/gui/TextAlignment#calculateLeft` | method | 1 |
| `net/minecraft/client/gui/components/BossHealthOverlay#events` | field | 1 |
| `net/minecraft/client/gui/components/ChatComponent#addClientSystemMessage` | method | 1 |
| `net/minecraft/client/gui/components/ChatComponent#addMessage` | method | 1 |
| `net/minecraft/client/gui/components/ChatComponent#allMessages` | field | 2 |
| `net/minecraft/client/gui/components/ChatComponent#captureClickableText` | method | 1 |
| `net/minecraft/client/gui/components/ChatComponent#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/components/ChatComponent#getHeight` | method | 1 |
| `net/minecraft/client/gui/components/ChatComponent#getScale` | method | 1 |
| `net/minecraft/client/gui/components/ChatComponent#getWidth` | method | 2 |
| `net/minecraft/client/gui/components/ChatComponent#logChatMessage` | method | 1 |
| `net/minecraft/client/gui/components/ChatComponent#refreshTrimmedMessages` | method | 2 |
| `net/minecraft/client/gui/components/ChatComponent#trimmedMessages` | field | 2 |
| `net/minecraft/client/gui/components/ChatComponent$DisplayMode#FOREGROUND` | field | 2 |
| `net/minecraft/client/gui/components/DebugScreenOverlay#getPingLogger` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#getValue` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setValue` | method | 1 |
| `net/minecraft/client/gui/components/LerpingBossEvent#getName` | method | 1 |
| `net/minecraft/client/gui/components/PlayerFaceExtractor#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/components/PlayerTabOverlay#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/components/PlayerTabOverlay#footer` | field | 1 |
| `net/minecraft/client/gui/components/PlayerTabOverlay#header` | field | 1 |
| `net/minecraft/client/gui/components/SplashRenderer#<init>` | method | 1 |
| `net/minecraft/client/gui/components/debug/DebugScreenDisplayer#addToGroup` | method | 1 |
| `net/minecraft/client/gui/components/debug/DebugScreenEntries#register` | method | 1 |
| `net/minecraft/client/gui/components/debug/DebugScreenEntryList#isOverlayVisible` | method | 1 |
| `net/minecraft/client/gui/components/events/ContainerEventHandler#getChildAt` | method | 1 |
| `net/minecraft/client/gui/components/events/GuiEventListener#isFocused` | method | 1 |
| `net/minecraft/client/gui/contextualbar/ContextualBar#extractBackground` | method | 2 |
| `net/minecraft/client/gui/contextualbar/ContextualBar#extractExperienceLevel` | method | 2 |
| `net/minecraft/client/gui/font/GlyphRenderTypes#createForColorTexture` | method | 1 |
| `net/minecraft/client/gui/font/GlyphRenderTypes#createForIntensityTexture` | method | 1 |
| `net/minecraft/client/gui/font/GlyphRenderTypes#select` | method | 1 |
| `net/minecraft/client/gui/font/glyphs/BakedGlyph#createGlyph` | method | 1 |
| `net/minecraft/client/gui/font/glyphs/BakedSheetGlyph$GlyphInstance#style` | method | 2 |
| `net/minecraft/client/gui/navigation/ScreenPosition#x` | method | 2 |
| `net/minecraft/client/gui/navigation/ScreenPosition#y` | method | 2 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#<init>` | method | 4 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#bottom` | method | 1 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#height` | method | 2 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#intersection` | method | 1 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#left` | method | 1 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#position` | method | 2 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#right` | method | 1 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#top` | method | 1 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#transformMaxBounds` | method | 1 |
| `net/minecraft/client/gui/navigation/ScreenRectangle#width` | method | 2 |
| `net/minecraft/client/gui/render/GuiRenderer#CLEAR_COLOR` | field | 3 |
| `net/minecraft/client/gui/render/GuiRenderer#addElementToMesh` | method | 1 |
| `net/minecraft/client/gui/render/GuiRenderer#endFrame` | method | 1 |
| `net/minecraft/client/gui/render/GuiRenderer#executeDrawRange` | method | 2 |
| `net/minecraft/client/gui/render/GuiRenderer#preparePictureInPicture` | method | 2 |
| `net/minecraft/client/gui/render/GuiRenderer#preparePictureInPictureState` | method | 1 |
| `net/minecraft/client/gui/render/GuiRenderer#render` | method | 1 |
| `net/minecraft/client/gui/render/TextureSetup#noTexture` | method | 3 |
| `net/minecraft/client/gui/render/TextureSetup#singleTexture` | method | 3 |
| `net/minecraft/client/gui/render/pip/PictureInPictureRenderer#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/ChatScreen#input` | field | 1 |
| `net/minecraft/client/gui/screens/ChatScreen#onClose` | method | 1 |
| `net/minecraft/client/gui/screens/MenuScreens$ScreenConstructor#fromPacket` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#<init>` | method | 3 |
| `net/minecraft/client/gui/screens/Screen#charTyped` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#extractRenderState` | method | 3 |
| `net/minecraft/client/gui/screens/Screen#extractRenderStateWithTooltipAndSubtitles` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#init` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#keyPressed` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#mouseClicked` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#mouseDragged` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#mouseMoved` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#mouseReleased` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#mouseScrolled` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#onClose` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#removed` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#extractBackground` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#extractContents` | method | 4 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#extractLabels` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#extractRenderState` | method | 5 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#extractSlot` | method | 2 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#extractSlotHighlightBack` | method | 3 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#extractSlotHighlightFront` | method | 2 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#extractTooltip` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#getMenu` | method | 3 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#height` | field | 4 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#hoveredSlot` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#keyPressed` | method | 2 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#mouseClicked` | method | 2 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#onClose` | method | 2 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#slotClicked` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractContainerScreen#width` | field | 4 |
| `net/minecraft/client/gui/screens/inventory/AbstractSignEditScreen#line` | field | 3 |
| `net/minecraft/client/gui/screens/inventory/AbstractSignEditScreen#setMessage` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/AbstractSignEditScreen#text` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/ContainerScreen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/ContainerScreen#extractBackground` | method | 2 |
| `net/minecraft/client/gui/screens/inventory/ContainerScreen#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/ContainerScreen#getMenu` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/ContainerScreen#height` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/ContainerScreen#init` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/ContainerScreen#leftPos` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/ContainerScreen#removed` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/ContainerScreen#topPos` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/ContainerScreen#width` | field | 1 |
| `net/minecraft/client/gui/screens/inventory/InventoryScreen#extractEntityInInventoryFollowsMouse` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/MenuAccess#getMenu` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent#create` | method | 1 |
| `net/minecraft/client/gui/screens/inventory/tooltip/DefaultTooltipPositioner#INSTANCE` | field | 1 |
| `net/minecraft/client/input/CharacterEvent#codepoint` | method | 3 |
| `net/minecraft/client/input/InputQuirks#REPLACE_CTRL_KEY_WITH_CMD_KEY` | field | 1 |
| `net/minecraft/client/input/KeyEvent#<init>` | method | 2 |
| `net/minecraft/client/input/KeyEvent#input` | method | 1 |
| `net/minecraft/client/input/KeyEvent#key` | method | 3 |
| `net/minecraft/client/input/KeyEvent#scancode` | method | 1 |
| `net/minecraft/client/input/MouseButtonEvent#button` | method | 3 |
| `net/minecraft/client/input/MouseButtonEvent#x` | method | 2 |
| `net/minecraft/client/input/MouseButtonEvent#y` | method | 2 |
| `net/minecraft/client/input/MouseButtonInfo#button` | method | 1 |
| `net/minecraft/client/model/Model#renderToBuffer` | method | 1 |
| `net/minecraft/client/model/geom/ModelPart#render` | method | 1 |
| `net/minecraft/client/multiplayer/ClientChunkCache#hasChunk` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#addBreakingBlockEffect` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#addDestroyBlockEffect` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#addEntity` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#addParticle` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#blockEntityTickers` | field | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#clip` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#damageSources` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#dimensionType` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#entitiesForRendering` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getBlockEntity` | method | 3 |
| `net/minecraft/client/multiplayer/ClientLevel#getBlockState` | method | 3 |
| `net/minecraft/client/multiplayer/ClientLevel#getChunkSource` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getDefaultClockTime` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getEntitiesOfClass` | method | 39 |
| `net/minecraft/client/multiplayer/ClientLevel#getEntity` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#getPlayerByUUID` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getScoreboard` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#players` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#registryAccess` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#getOnlinePlayers` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#getPlayerInfo` | method | 2 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleParticleEvent` | method | 2 |
| `net/minecraft/client/multiplayer/ClientPacketListener#sendCommand` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#tick` | method | 1 |
| `net/minecraft/client/multiplayer/MultiPlayerGameMode#destroyBlockPos` | field | 1 |
| `net/minecraft/client/multiplayer/MultiPlayerGameMode#handleContainerInput` | method | 1 |
| `net/minecraft/client/multiplayer/MultiPlayerGameMode#isDestroying` | method | 1 |
| `net/minecraft/client/multiplayer/PlayerInfo#getGameMode` | method | 1 |
| `net/minecraft/client/multiplayer/PlayerInfo#getProfile` | method | 3 |
| `net/minecraft/client/multiplayer/PlayerInfo#getSkin` | method | 1 |
| `net/minecraft/client/multiplayer/PlayerInfo#getTeam` | method | 1 |
| `net/minecraft/client/multiplayer/PlayerInfo#showHat` | method | 1 |
| `net/minecraft/client/multiplayer/chat/GuiMessage#<init>` | method | 2 |
| `net/minecraft/client/multiplayer/chat/GuiMessage#addedTime` | method | 2 |
| `net/minecraft/client/multiplayer/chat/GuiMessage#content` | method | 4 |
| `net/minecraft/client/multiplayer/chat/GuiMessage#signature` | method | 2 |
| `net/minecraft/client/multiplayer/chat/GuiMessage#splitLines` | method | 1 |
| `net/minecraft/client/multiplayer/chat/GuiMessage$Line#<init>` | method | 1 |
| `net/minecraft/client/multiplayer/chat/GuiMessage$Line#content` | method | 1 |
| `net/minecraft/client/multiplayer/chat/GuiMessage$Line#parent` | method | 2 |
| `net/minecraft/client/multiplayer/chat/GuiMessageSource#SYSTEM_CLIENT` | field | 3 |
| `net/minecraft/client/multiplayer/chat/GuiMessageTag#system` | method | 3 |
| `net/minecraft/client/player/AbstractClientPlayer#getName` | method | 1 |
| `net/minecraft/client/player/AbstractClientPlayer#getSkin` | method | 1 |
| `net/minecraft/client/player/AbstractClientPlayer#isModelPartShown` | method | 1 |
| `net/minecraft/client/player/AbstractClientPlayer#level` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#blockPosition` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#closeContainer` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#connection` | field | 2 |
| `net/minecraft/client/player/LocalPlayer#containerMenu` | field | 5 |
| `net/minecraft/client/player/LocalPlayer#experienceLevel` | field | 4 |
| `net/minecraft/client/player/LocalPlayer#experienceProgress` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#getAbilities` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getAttributeBaseValue` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getDirection` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getEyeHeight` | method | 5 |
| `net/minecraft/client/player/LocalPlayer#getEyePosition` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getInventory` | method | 4 |
| `net/minecraft/client/player/LocalPlayer#getLookAngle` | method | 5 |
| `net/minecraft/client/player/LocalPlayer#getMainHandItem` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getName` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getPose` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getX` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getXRot` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getY` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getYRot` | method | 2 |
| `net/minecraft/client/player/LocalPlayer#getZ` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isAlive` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isFallFlying` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isInWater` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isPassenger` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#position` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#sendSystemMessage` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#setExperienceValues` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#totalExperience` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#xOld` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#yOld` | field | 1 |
| `net/minecraft/client/player/LocalPlayer#zOld` | field | 1 |
| `net/minecraft/client/player/RemotePlayer#blockPosition` | method | 1 |
| `net/minecraft/client/player/RemotePlayer#getId` | method | 2 |
| `net/minecraft/client/player/RemotePlayer#getName` | method | 12 |
| `net/minecraft/client/player/RemotePlayer#getVehicle` | method | 1 |
| `net/minecraft/client/renderer/BindGroupLayouts#MATRICES_PROJECTION` | field | 1 |
| `net/minecraft/client/renderer/DynamicUniformStorage#<init>` | method | 1 |
| `net/minecraft/client/renderer/DynamicUniformStorage#close` | method | 1 |
| `net/minecraft/client/renderer/DynamicUniformStorage#endFrame` | method | 1 |
| `net/minecraft/client/renderer/DynamicUniformStorage#writeUniform` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#gameRenderState` | method | 3 |
| `net/minecraft/client/renderer/GameRenderer#lighting` | method | 2 |
| `net/minecraft/client/renderer/GameRenderer#mainCamera` | method | 2 |
| `net/minecraft/client/renderer/OrderedSubmitNodeCollector#submitModel` | method | 1 |
| `net/minecraft/client/renderer/Projection#<init>` | method | 2 |
| `net/minecraft/client/renderer/Projection#setupOrtho` | method | 2 |
| `net/minecraft/client/renderer/ProjectionMatrixBuffer#<init>` | method | 1 |
| `net/minecraft/client/renderer/ProjectionMatrixBuffer#close` | method | 1 |
| `net/minecraft/client/renderer/ProjectionMatrixBuffer#getBuffer` | method | 2 |
| `net/minecraft/client/renderer/RenderPipelines#DEBUG_FILLED_SNIPPET` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_INVERT` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_SNIPPET` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_TEXTURED` | field | 4 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_TEXT_HIGHLIGHT` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#LINES_SNIPPET` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#register` | method | 1 |
| `net/minecraft/client/renderer/ScreenEffectRenderer#submitBlockSprite` | method | 1 |
| `net/minecraft/client/renderer/ScreenEffectRenderer#submitFire` | method | 1 |
| `net/minecraft/client/renderer/ScreenEffectRenderer#submitWater` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitCustom` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitCustomGeometry` | method | 3 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitModel` | method | 2 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitNameTag` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitText` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeStorage#<init>` | method | 1 |
| `net/minecraft/client/renderer/SubmitNodeStorage#hashCode` | method | 1 |
| `net/minecraft/client/renderer/block/BlockStateModelSet#get` | method | 2 |
| `net/minecraft/client/renderer/block/FluidModel#<init>` | method | 1 |
| `net/minecraft/client/renderer/block/FluidModel#flowingMaterial` | method | 1 |
| `net/minecraft/client/renderer/block/FluidModel#overlayMaterial` | method | 1 |
| `net/minecraft/client/renderer/block/FluidModel#stillMaterial` | method | 1 |
| `net/minecraft/client/renderer/block/FluidModel#tintSource` | method | 1 |
| `net/minecraft/client/renderer/block/FluidStateModelSet#bake` | method | 1 |
| `net/minecraft/client/renderer/block/FluidStateModelSet#get` | method | 1 |
| `net/minecraft/client/renderer/blockentity/BeaconRenderer#submitBeaconBeam` | method | 1 |
| `net/minecraft/client/renderer/blockentity/SkullBlockRenderer#submitSkull` | method | 1 |
| `net/minecraft/client/renderer/chunk/ChunkSectionLayer#SOLID` | field | 1 |
| `net/minecraft/client/renderer/culling/Frustum#isVisible` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderDispatcher#getRenderer` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderDispatcher#shouldRender` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderDispatcher#submit` | method | 3 |
| `net/minecraft/client/renderer/entity/EntityRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderer#createRenderState` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderer#extractRenderState` | method | 2 |
| `net/minecraft/client/renderer/entity/EntityRenderer#submitNameDisplay` | method | 3 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#extractRenderState` | method | 1 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#getRenderType` | method | 1 |
| `net/minecraft/client/renderer/entity/LivingEntityRenderer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/CapeLayer#submit` | method | 2 |
| `net/minecraft/client/renderer/entity/layers/CustomHeadLayer#submit` | method | 2 |
| `net/minecraft/client/renderer/entity/layers/EquipmentLayerRenderer#renderLayers` | method | 1 |
| `net/minecraft/client/renderer/entity/layers/HumanoidArmorLayer#renderArmorPiece` | method | 1 |
| `net/minecraft/client/renderer/entity/player/AvatarRenderer#isEntityUpsideDown` | method | 1 |
| `net/minecraft/client/renderer/entity/state/EntityRenderState#lightCoords` | field | 1 |
| `net/minecraft/client/renderer/entity/state/EntityRenderState#skyhanni$getEntity` | method | 1 |
| `net/minecraft/client/renderer/entity/state/EntityRenderState#skyhanni$setEntity` | method | 2 |
| `net/minecraft/client/renderer/entity/state/EntityRenderState#x` | field | 1 |
| `net/minecraft/client/renderer/entity/state/EntityRenderState#y` | field | 1 |
| `net/minecraft/client/renderer/entity/state/EntityRenderState#z` | field | 1 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#bodyRot` | field | 1 |
| `net/minecraft/client/renderer/entity/state/LivingEntityRenderState#isBaby` | field | 1 |
| `net/minecraft/client/renderer/extract/LevelExtractor#allChanged` | method | 1 |
| `net/minecraft/client/renderer/extract/LevelExtractor#extractVisibleEntities` | method | 1 |
| `net/minecraft/client/renderer/feature/FeatureRenderDispatcher#hashCode` | method | 1 |
| `net/minecraft/client/renderer/feature/FeatureRenderDispatcher#renderAllFeatures` | method | 1 |
| `net/minecraft/client/renderer/feature/ItemFeatureRenderer#prepareOutlineSubmit` | method | 2 |
| `net/minecraft/client/renderer/feature/TextFeatureRenderer$Submit#<init>` | method | 1 |
| `net/minecraft/client/renderer/item/ItemModelResolver#updateForTopItem` | method | 1 |
| `net/minecraft/client/renderer/item/TrackingItemStackRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/item/TrackingItemStackRenderState#appendModelIdentityElement` | method | 1 |
| `net/minecraft/client/renderer/item/TrackingItemStackRenderState#getModelIdentity` | method | 1 |
| `net/minecraft/client/renderer/item/TrackingItemStackRenderState#isAnimated` | method | 1 |
| `net/minecraft/client/renderer/item/TrackingItemStackRenderState#setAnimated` | method | 1 |
| `net/minecraft/client/renderer/item/TrackingItemStackRenderState#submit` | method | 1 |
| `net/minecraft/client/renderer/item/TrackingItemStackRenderState#usesBlockLight` | method | 2 |
| `net/minecraft/client/renderer/rendertype/LayeringTransform#NO_LAYERING` | field | 1 |
| `net/minecraft/client/renderer/rendertype/LayeringTransform#VIEW_OFFSET_Z_LAYERING` | field | 1 |
| `net/minecraft/client/renderer/rendertype/PreparedRenderType#drawFromBuffer` | method | 1 |
| `net/minecraft/client/renderer/rendertype/PreparedRenderType#pipeline` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup#builder` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#createRenderSetup` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#setLayeringTransform` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#withTexture` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#create` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#armorTranslucent` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#entityTranslucentCullItemTarget` | method | 2 |
| `net/minecraft/client/renderer/rendertype/RenderTypes#glintTranslucent` | method | 1 |
| `net/minecraft/client/renderer/state/GameRenderState#guiRenderState` | field | 2 |
| `net/minecraft/client/renderer/state/GameRenderState#levelRenderState` | field | 1 |
| `net/minecraft/client/renderer/state/gui/BlitRenderState#<init>` | method | 2 |
| `net/minecraft/client/renderer/state/gui/BlitRenderState#color` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GlyphRenderState#renderable` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiElementRenderState#pipeline` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiItemRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiItemRenderState#bounds` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiItemRenderState#hashCode` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiItemRenderState#itemStackRenderState` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiItemRenderState#pose` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiItemRenderState#scissorArea` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiRenderState#addBlitToCurrentLayer` | method | 2 |
| `net/minecraft/client/renderer/state/gui/GuiRenderState#addGuiElement` | method | 2 |
| `net/minecraft/client/renderer/state/gui/GuiRenderState#addPicturesInPictureState` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiRenderState#addText` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiTextRenderState#<init>` | method | 2 |
| `net/minecraft/client/renderer/state/gui/GuiTextRenderState#backgroundColor` | field | 1 |
| `net/minecraft/client/renderer/state/gui/GuiTextRenderState#color` | field | 1 |
| `net/minecraft/client/renderer/state/gui/GuiTextRenderState#dropShadow` | field | 1 |
| `net/minecraft/client/renderer/state/gui/GuiTextRenderState#font` | field | 1 |
| `net/minecraft/client/renderer/state/gui/GuiTextRenderState#pose` | field | 1 |
| `net/minecraft/client/renderer/state/gui/GuiTextRenderState#scissor` | field | 1 |
| `net/minecraft/client/renderer/state/gui/GuiTextRenderState#text` | field | 1 |
| `net/minecraft/client/renderer/state/gui/GuiTextRenderState#x` | field | 1 |
| `net/minecraft/client/renderer/state/gui/GuiTextRenderState#y` | field | 1 |
| `net/minecraft/client/renderer/state/level/CameraRenderState#orientation` | field | 1 |
| `net/minecraft/client/renderer/state/level/CameraRenderState#pos` | field | 2 |
| `net/minecraft/client/renderer/state/level/LevelRenderState#cameraRenderState` | field | 2 |
| `net/minecraft/client/renderer/texture/AbstractTexture#<init>` | method | 1 |
| `net/minecraft/client/renderer/texture/AbstractTexture#getSampler` | method | 1 |
| `net/minecraft/client/renderer/texture/AbstractTexture#getTextureView` | method | 2 |
| `net/minecraft/client/renderer/texture/DynamicTexture#<init>` | method | 2 |
| `net/minecraft/client/renderer/texture/DynamicTexture#getPixels` | method | 1 |
| `net/minecraft/client/renderer/texture/DynamicTexture#upload` | method | 3 |
| `net/minecraft/client/renderer/texture/OverlayTexture#NO_OVERLAY` | field | 1 |
| `net/minecraft/client/renderer/texture/TextureManager#getTexture` | method | 2 |
| `net/minecraft/client/renderer/texture/TextureManager#register` | method | 3 |
| `net/minecraft/client/renderer/texture/TextureManager#release` | method | 1 |
| `net/minecraft/client/resources/SplashManager#getSplash` | method | 1 |
| `net/minecraft/client/resources/language/I18n#get` | method | 3 |
| `net/minecraft/client/resources/sounds/SimpleSoundInstance#<init>` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#createUnseededRandom` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getIdentifier` | method | 2 |
| `net/minecraft/client/resources/sounds/SoundInstance#getPitch` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getVolume` | method | 2 |
| `net/minecraft/client/resources/sounds/SoundInstance#getX` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getY` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getZ` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance$Attenuation#NONE` | field | 1 |
| `net/minecraft/client/sounds/SoundEngine#calculateVolume` | method | 1 |
| `net/minecraft/client/sounds/SoundEngine#play` | method | 2 |
| `net/minecraft/client/sounds/SoundEngine$PlayResult#NOT_STARTED` | field | 1 |
| `net/minecraft/client/sounds/SoundManager#play` | method | 1 |
| `net/minecraft/core/BlockPos#<init>` | method | 2 |
| `net/minecraft/core/BlockPos#betweenClosed` | method | 1 |
| `net/minecraft/core/BlockPos#getX` | method | 2 |
| `net/minecraft/core/BlockPos#getY` | method | 2 |
| `net/minecraft/core/BlockPos#getZ` | method | 2 |
| `net/minecraft/core/BlockPos#offset` | method | 1 |
| `net/minecraft/core/ClientAsset$Texture#texturePath` | method | 2 |
| `net/minecraft/core/DefaultedRegistry#getKey` | method | 3 |
| `net/minecraft/core/DefaultedRegistry#getValue` | method | 5 |
| `net/minecraft/core/DefaultedRegistry#wrapAsHolder` | method | 1 |
| `net/minecraft/core/Direction#DOWN` | field | 4 |
| `net/minecraft/core/Direction#EAST` | field | 3 |
| `net/minecraft/core/Direction#NORTH` | field | 3 |
| `net/minecraft/core/Direction#SOUTH` | field | 3 |
| `net/minecraft/core/Direction#UP` | field | 3 |
| `net/minecraft/core/Direction#WEST` | field | 3 |
| `net/minecraft/core/Direction#getStepX` | method | 1 |
| `net/minecraft/core/Direction#getStepY` | method | 1 |
| `net/minecraft/core/Direction#getStepZ` | method | 1 |
| `net/minecraft/core/Direction#getUnitVec3i` | method | 2 |
| `net/minecraft/core/Direction#ordinal` | method | 4 |
| `net/minecraft/core/Direction#toString` | method | 1 |
| `net/minecraft/core/Direction#values` | method | 4 |
| `net/minecraft/core/Direction$Axis#X` | field | 6 |
| `net/minecraft/core/Direction$Axis#Y` | field | 7 |
| `net/minecraft/core/Direction$Axis#Z` | field | 4 |
| `net/minecraft/core/Direction$Axis#name` | method | 1 |
| `net/minecraft/core/Direction$Axis#ordinal` | method | 2 |
| `net/minecraft/core/Direction$Axis#values` | method | 7 |
| `net/minecraft/core/Holder#areComponentsBound` | method | 1 |
| `net/minecraft/core/Holder#components` | method | 1 |
| `net/minecraft/core/Holder#direct` | method | 1 |
| `net/minecraft/core/Holder#value` | method | 1 |
| `net/minecraft/core/NonNullList#get` | method | 5 |
| `net/minecraft/core/NonNullList#indexOf` | method | 1 |
| `net/minecraft/core/NonNullList#iterator` | method | 8 |
| `net/minecraft/core/Registry#containsKey` | method | 1 |
| `net/minecraft/core/Registry#get` | method | 1 |
| `net/minecraft/core/Registry#getKey` | method | 2 |
| `net/minecraft/core/Registry#keySet` | method | 2 |
| `net/minecraft/core/RegistryAccess#lookupOrThrow` | method | 1 |
| `net/minecraft/core/Rotations#x` | method | 2 |
| `net/minecraft/core/Rotations#y` | method | 2 |
| `net/minecraft/core/Rotations#z` | method | 2 |
| `net/minecraft/core/Vec3i#getX` | method | 1 |
| `net/minecraft/core/Vec3i#getZ` | method | 1 |
| `net/minecraft/core/component/DataComponentMap#EMPTY` | field | 1 |
| `net/minecraft/core/component/DataComponentMap#get` | method | 1 |
| `net/minecraft/core/component/DataComponentMap#isEmpty` | method | 1 |
| `net/minecraft/core/component/DataComponentMap#keySet` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch#EMPTY` | field | 1 |
| `net/minecraft/core/component/DataComponentPatch#builder` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch#entrySet` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch$Builder#build` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch$Builder#remove` | method | 1 |
| `net/minecraft/core/component/DataComponentPatch$Builder#set` | method | 1 |
| `net/minecraft/core/component/DataComponents#ATTRIBUTE_MODIFIERS` | field | 1 |
| `net/minecraft/core/component/DataComponents#CUSTOM_DATA` | field | 5 |
| `net/minecraft/core/component/DataComponents#CUSTOM_NAME` | field | 2 |
| `net/minecraft/core/component/DataComponents#DAMAGE` | field | 1 |
| `net/minecraft/core/component/DataComponents#DYED_COLOR` | field | 1 |
| `net/minecraft/core/component/DataComponents#ENCHANTMENTS` | field | 2 |
| `net/minecraft/core/component/DataComponents#ENCHANTMENT_GLINT_OVERRIDE` | field | 3 |
| `net/minecraft/core/component/DataComponents#ITEM_MODEL` | field | 2 |
| `net/minecraft/core/component/DataComponents#LORE` | field | 1 |
| `net/minecraft/core/component/DataComponents#PROFILE` | field | 2 |
| `net/minecraft/core/component/DataComponents#TOOLTIP_DISPLAY` | field | 1 |
| `net/minecraft/core/component/DataComponents#UNBREAKABLE` | field | 2 |
| `net/minecraft/core/component/PatchedDataComponentMap#applyPatch` | method | 1 |
| `net/minecraft/core/component/PatchedDataComponentMap#fromPatch` | method | 1 |
| `net/minecraft/core/component/TypedDataComponent#type` | method | 1 |
| `net/minecraft/core/particles/ColorParticleOption#color` | field | 1 |
| `net/minecraft/core/particles/ColorParticleOption#getAlpha` | method | 1 |
| `net/minecraft/core/particles/ParticleOptions#getType` | method | 2 |
| `net/minecraft/core/particles/ParticleTypes#ANGRY_VILLAGER` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#BLOCK` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#CLOUD` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#CRIT` | field | 3 |
| `net/minecraft/core/particles/ParticleTypes#DRIPPING_LAVA` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#DUST` | field | 7 |
| `net/minecraft/core/particles/ParticleTypes#ENCHANT` | field | 4 |
| `net/minecraft/core/particles/ParticleTypes#ENCHANTED_HIT` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#ENTITY_EFFECT` | field | 3 |
| `net/minecraft/core/particles/ParticleTypes#EXPLOSION` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#EXPLOSION_EMITTER` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#FIREWORK` | field | 3 |
| `net/minecraft/core/particles/ParticleTypes#FLAME` | field | 4 |
| `net/minecraft/core/particles/ParticleTypes#HAPPY_VILLAGER` | field | 4 |
| `net/minecraft/core/particles/ParticleTypes#HEART` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#LARGE_SMOKE` | field | 2 |
| `net/minecraft/core/particles/ParticleTypes#POOF` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#SMOKE` | field | 1 |
| `net/minecraft/core/particles/ParticleTypes#WITCH` | field | 3 |
| `net/minecraft/core/registries/BuiltInRegistries#ENTITY_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#PARTICLE_TYPE` | field | 3 |
| `net/minecraft/core/registries/BuiltInRegistries#SOUND_EVENT` | field | 1 |
| `net/minecraft/core/registries/Registries#ENCHANTMENT` | field | 1 |
| `net/minecraft/locale/Language#getInstance` | method | 1 |
| `net/minecraft/locale/Language#getVisualOrder` | method | 1 |
| `net/minecraft/nbt/CompoundTag#<init>` | method | 3 |
| `net/minecraft/nbt/CompoundTag#contains` | method | 5 |
| `net/minecraft/nbt/CompoundTag#get` | method | 3 |
| `net/minecraft/nbt/CompoundTag#getBoolean` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getByte` | method | 2 |
| `net/minecraft/nbt/CompoundTag#getCompound` | method | 2 |
| `net/minecraft/nbt/CompoundTag#getDouble` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getFloat` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getInt` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getList` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getLong` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getString` | method | 2 |
| `net/minecraft/nbt/CompoundTag#hashCode` | method | 1 |
| `net/minecraft/nbt/CompoundTag#isEmpty` | method | 1 |
| `net/minecraft/nbt/CompoundTag#keySet` | method | 4 |
| `net/minecraft/nbt/CompoundTag#putString` | method | 1 |
| `net/minecraft/nbt/CompoundTag#toString` | method | 1 |
| `net/minecraft/nbt/ListTag#<init>` | method | 1 |
| `net/minecraft/nbt/ListTag#getCompound` | method | 1 |
| `net/minecraft/nbt/ListTag#getString` | method | 1 |
| `net/minecraft/nbt/ListTag#isEmpty` | method | 1 |
| `net/minecraft/nbt/ListTag#iterator` | method | 1 |
| `net/minecraft/nbt/ListTag#size` | method | 2 |
| `net/minecraft/nbt/NbtAccounter#unlimitedHeap` | method | 2 |
| `net/minecraft/nbt/NbtIo#readCompressed` | method | 2 |
| `net/minecraft/nbt/NbtOps#INSTANCE` | field | 1 |
| `net/minecraft/nbt/NbtOps#convertTo` | method | 1 |
| `net/minecraft/nbt/StringTag#value` | method | 1 |
| `net/minecraft/nbt/StringTag#valueOf` | method | 1 |
| `net/minecraft/nbt/Tag#asCompound` | method | 1 |
| `net/minecraft/network/Connection#genericsFtw` | method | 1 |
| `net/minecraft/network/Connection#send` | method | 1 |
| `net/minecraft/network/chat/ClickEvent#action` | method | 3 |
| `net/minecraft/network/chat/ClickEvent$Action#CHANGE_PAGE` | field | 1 |
| `net/minecraft/network/chat/ClickEvent$Action#COPY_TO_CLIPBOARD` | field | 1 |
| `net/minecraft/network/chat/ClickEvent$Action#OPEN_FILE` | field | 1 |
| `net/minecraft/network/chat/ClickEvent$Action#OPEN_URL` | field | 2 |
| `net/minecraft/network/chat/ClickEvent$Action#RUN_COMMAND` | field | 2 |
| `net/minecraft/network/chat/ClickEvent$Action#SUGGEST_COMMAND` | field | 2 |
| `net/minecraft/network/chat/ClickEvent$Action#name` | method | 1 |
| `net/minecraft/network/chat/ClickEvent$Action#ordinal` | method | 2 |
| `net/minecraft/network/chat/ClickEvent$Action#values` | method | 1 |
| `net/minecraft/network/chat/ClickEvent$ChangePage#page` | method | 1 |
| `net/minecraft/network/chat/ClickEvent$CopyToClipboard#value` | method | 1 |
| `net/minecraft/network/chat/ClickEvent$OpenFile#path` | method | 1 |
| `net/minecraft/network/chat/ClickEvent$OpenUrl#<init>` | method | 2 |
| `net/minecraft/network/chat/ClickEvent$OpenUrl#uri` | method | 1 |
| `net/minecraft/network/chat/ClickEvent$RunCommand#<init>` | method | 2 |
| `net/minecraft/network/chat/ClickEvent$RunCommand#command` | method | 1 |
| `net/minecraft/network/chat/ClickEvent$SuggestCommand#<init>` | method | 1 |
| `net/minecraft/network/chat/ClickEvent$SuggestCommand#command` | method | 1 |
| `net/minecraft/network/chat/Component#copy` | method | 14 |
| `net/minecraft/network/chat/Component#empty` | method | 22 |
| `net/minecraft/network/chat/Component#getContents` | method | 2 |
| `net/minecraft/network/chat/Component#getSiblings` | method | 18 |
| `net/minecraft/network/chat/Component#getString` | method | 162 |
| `net/minecraft/network/chat/Component#getStyle` | method | 16 |
| `net/minecraft/network/chat/Component#getVisualOrderText` | method | 2 |
| `net/minecraft/network/chat/Component#literal` | method | 36 |
| `net/minecraft/network/chat/Component#object` | method | 2 |
| `net/minecraft/network/chat/Component#translatable` | method | 1 |
| `net/minecraft/network/chat/Component#visit` | method | 4 |
| `net/minecraft/network/chat/ComponentSerialization#CODEC` | field | 3 |
| `net/minecraft/network/chat/FormattedText#visit` | method | 2 |
| `net/minecraft/network/chat/HoverEvent#action` | method | 3 |
| `net/minecraft/network/chat/HoverEvent$Action#SHOW_ENTITY` | field | 1 |
| `net/minecraft/network/chat/HoverEvent$Action#SHOW_ITEM` | field | 2 |
| `net/minecraft/network/chat/HoverEvent$Action#SHOW_TEXT` | field | 3 |
| `net/minecraft/network/chat/HoverEvent$Action#name` | method | 1 |
| `net/minecraft/network/chat/HoverEvent$Action#ordinal` | method | 2 |
| `net/minecraft/network/chat/HoverEvent$Action#values` | method | 1 |
| `net/minecraft/network/chat/HoverEvent$EntityTooltipInfo#name` | field | 1 |
| `net/minecraft/network/chat/HoverEvent$ShowEntity#entity` | method | 1 |
| `net/minecraft/network/chat/HoverEvent$ShowItem#<init>` | method | 1 |
| `net/minecraft/network/chat/HoverEvent$ShowItem#item` | method | 1 |
| `net/minecraft/network/chat/HoverEvent$ShowText#<init>` | method | 1 |
| `net/minecraft/network/chat/HoverEvent$ShowText#value` | method | 1 |
| `net/minecraft/network/chat/MessageSignature#<init>` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#create` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#getSiblings` | method | 2 |
| `net/minecraft/network/chat/MutableComponent#getString` | method | 2 |
| `net/minecraft/network/chat/MutableComponent#getStyle` | method | 2 |
| `net/minecraft/network/chat/MutableComponent#setStyle` | method | 11 |
| `net/minecraft/network/chat/MutableComponent#skyhanni$didCreate` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#skyhanni$setCreated` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#toFlatList` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#withColor` | method | 13 |
| `net/minecraft/network/chat/Style#EMPTY` | field | 11 |
| `net/minecraft/network/chat/Style#applyLegacyFormat` | method | 1 |
| `net/minecraft/network/chat/Style#getClickEvent` | method | 6 |
| `net/minecraft/network/chat/Style#getColor` | method | 12 |
| `net/minecraft/network/chat/Style#getHoverEvent` | method | 4 |
| `net/minecraft/network/chat/Style#hashCode` | method | 1 |
| `net/minecraft/network/chat/Style#isBold` | method | 5 |
| `net/minecraft/network/chat/Style#isEmpty` | method | 1 |
| `net/minecraft/network/chat/Style#isItalic` | method | 5 |
| `net/minecraft/network/chat/Style#isObfuscated` | method | 4 |
| `net/minecraft/network/chat/Style#isStrikethrough` | method | 5 |
| `net/minecraft/network/chat/Style#isUnderlined` | method | 4 |
| `net/minecraft/network/chat/Style#withBold` | method | 5 |
| `net/minecraft/network/chat/Style#withClickEvent` | method | 2 |
| `net/minecraft/network/chat/Style#withColor` | method | 14 |
| `net/minecraft/network/chat/Style#withHoverEvent` | method | 1 |
| `net/minecraft/network/chat/Style#withItalic` | method | 4 |
| `net/minecraft/network/chat/Style#withObfuscated` | method | 3 |
| `net/minecraft/network/chat/Style#withStrikethrough` | method | 4 |
| `net/minecraft/network/chat/Style#withUnderlined` | method | 3 |
| `net/minecraft/network/chat/TextColor#<init>` | method | 4 |
| `net/minecraft/network/chat/TextColor#equals` | method | 1 |
| `net/minecraft/network/chat/TextColor#fromLegacyFormat` | method | 2 |
| `net/minecraft/network/chat/TextColor#fromRgb` | method | 2 |
| `net/minecraft/network/chat/TextColor#getValue` | method | 2 |
| `net/minecraft/network/chat/TextColor#name` | field | 6 |
| `net/minecraft/network/chat/TextColor#serialize` | method | 1 |
| `net/minecraft/network/chat/contents/PlainTextContents#text` | method | 2 |
| `net/minecraft/network/chat/contents/objects/AtlasSprite#<init>` | method | 1 |
| `net/minecraft/network/chat/contents/objects/PlayerSprite#<init>` | method | 2 |
| `net/minecraft/network/protocol/BundlePacket#subPackets` | method | 1 |
| `net/minecraft/network/protocol/PacketUtils#ensureRunningOnSameThread` | method | 1 |
| `net/minecraft/network/protocol/common/ClientboundPingPacket#getId` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundAddEntityPacket#getId` | method | 2 |
| `net/minecraft/network/protocol/game/ClientboundAddEntityPacket#getX` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundAddEntityPacket#getY` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundAddEntityPacket#getZ` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundAnimatePacket#getId` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundBlockUpdatePacket#getBlockState` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundBlockUpdatePacket#getPos` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundContainerSetContentPacket#containerId` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundContainerSetContentPacket#items` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundContainerSetSlotPacket#getContainerId` | method | 2 |
| `net/minecraft/network/protocol/game/ClientboundContainerSetSlotPacket#getItem` | method | 2 |
| `net/minecraft/network/protocol/game/ClientboundContainerSetSlotPacket#getSlot` | method | 2 |
| `net/minecraft/network/protocol/game/ClientboundEntityEventPacket#getEntity` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundHurtAnimationPacket#id` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundInitializeBorderPacket#getWarningTime` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLevelEventPacket#getPos` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLevelParticlesPacket#getCount` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLevelParticlesPacket#getMaxSpeed` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLevelParticlesPacket#getParticle` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLevelParticlesPacket#getX` | method | 2 |
| `net/minecraft/network/protocol/game/ClientboundLevelParticlesPacket#getXDist` | method | 2 |
| `net/minecraft/network/protocol/game/ClientboundLevelParticlesPacket#getY` | method | 2 |
| `net/minecraft/network/protocol/game/ClientboundLevelParticlesPacket#getYDist` | method | 2 |
| `net/minecraft/network/protocol/game/ClientboundLevelParticlesPacket#getZ` | method | 2 |
| `net/minecraft/network/protocol/game/ClientboundLevelParticlesPacket#getZDist` | method | 2 |
| `net/minecraft/network/protocol/game/ClientboundLevelParticlesPacket#isOverrideLimiter` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLevelParticlesPacket#skyhanni$setShouldSuppress` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLevelParticlesPacket#skyhanni$shouldSuppress` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundMoveEntityPacket#getEntity` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundOpenScreenPacket#getContainerId` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundOpenScreenPacket#getTitle` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundOpenScreenPacket#getType` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundPlayerPositionPacket#change` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundPlayerPositionPacket#relatives` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundRemoveEntitiesPacket#getEntityIds` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSectionBlocksUpdatePacket#runUpdates` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSetEntityDataPacket#id` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSetEntityLinkPacket#getSourceId` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSetEntityMotionPacket#id` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSetEquipmentPacket#getEntity` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSetObjectivePacket#getDisplayName` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSetObjectivePacket#getObjectiveName` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSetObjectivePacket#getRenderType` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSetPlayerTeamPacket#getName` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSetScorePacket#objectiveName` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSetTimePacket#clockUpdates` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundTeleportEntityPacket#id` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundUpdateAttributesPacket#getEntityId` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundUpdateMobEffectPacket#getEntityId` | method | 1 |
| `net/minecraft/network/protocol/game/ServerboundAttackPacket#entityId` | method | 1 |
| `net/minecraft/network/protocol/game/ServerboundChatCommandPacket#command` | method | 1 |
| `net/minecraft/network/protocol/game/ServerboundChatPacket#message` | method | 1 |
| `net/minecraft/network/protocol/game/ServerboundInteractPacket#entityId` | field | 1 |
| `net/minecraft/network/protocol/game/ServerboundMovePlayerPacket#getX` | method | 1 |
| `net/minecraft/network/protocol/game/ServerboundMovePlayerPacket#getY` | method | 1 |
| `net/minecraft/network/protocol/game/ServerboundMovePlayerPacket#getZ` | method | 1 |
| `net/minecraft/network/protocol/game/VecDeltaCodec#getBase` | method | 1 |
| `net/minecraft/network/syncher/SynchedEntityData#assignValues` | method | 1 |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem#getAccessor` | method | 1 |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem#getValue` | method | 1 |
| `net/minecraft/network/syncher/SynchedEntityData$DataValue#id` | method | 1 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 12 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 1 |
| `net/minecraft/resources/Identifier#getPath` | method | 2 |
| `net/minecraft/resources/Identifier#parse` | method | 6 |
| `net/minecraft/resources/Identifier#toDebugFileName` | method | 1 |
| `net/minecraft/resources/Identifier#toString` | method | 5 |
| `net/minecraft/resources/Identifier#tryParse` | method | 1 |
| `net/minecraft/resources/Identifier#withDefaultNamespace` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#getName` | method | 1 |
| `net/minecraft/server/packs/PackType#CLIENT_RESOURCES` | field | 1 |
| `net/minecraft/server/packs/resources/PreparableReloadListener$PreparationBarrier#wait` | method | 1 |
| `net/minecraft/server/packs/resources/PreparableReloadListener$SharedState#resourceManager` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#open` | method | 4 |
| `net/minecraft/server/packs/resources/ResourceManager#getResource` | method | 3 |
| `net/minecraft/server/packs/resources/ResourceManager#getResourceOrThrow` | method | 1 |
| `net/minecraft/sounds/SoundSource#UI` | field | 1 |
| `net/minecraft/util/ARGB#alpha` | method | 1 |
| `net/minecraft/util/ARGB#blue` | method | 1 |
| `net/minecraft/util/ARGB#color` | method | 1 |
| `net/minecraft/util/ARGB#green` | method | 1 |
| `net/minecraft/util/ARGB#red` | method | 1 |
| `net/minecraft/util/FormattedCharSequence#accept` | method | 3 |
| `net/minecraft/util/FormattedCharSequence#composite` | method | 1 |
| `net/minecraft/util/FormattedCharSequence#forward` | method | 1 |
| `net/minecraft/util/FormattedCharSink#accept` | method | 1 |
| `net/minecraft/util/LightCoordsUtil#getLightCoords` | method | 1 |
| `net/minecraft/util/RandomSource#nextInt` | method | 1 |
| `net/minecraft/util/StringDecomposer#feedChar` | method | 1 |
| `net/minecraft/util/StringDecomposer#iterateFormatted` | method | 7 |
| `net/minecraft/util/Unit#INSTANCE` | field | 1 |
| `net/minecraft/util/Util#getPlatform` | method | 2 |
| `net/minecraft/util/Util#memoize` | method | 1 |
| `net/minecraft/util/Util$OS#OSX` | field | 1 |
| `net/minecraft/util/Util$OS#openUri` | method | 1 |
| `net/minecraft/util/debugchart/LocalSampleLogger#get` | method | 1 |
| `net/minecraft/util/debugchart/LocalSampleLogger#size` | method | 1 |
| `net/minecraft/world/Container#getContainerSize` | method | 2 |
| `net/minecraft/world/Container#getItem` | method | 2 |
| `net/minecraft/world/InteractionResult#FAIL` | field | 1 |
| `net/minecraft/world/InteractionResult#PASS` | field | 1 |
| `net/minecraft/world/InteractionResult#SUCCESS` | field | 1 |
| `net/minecraft/world/SimpleContainer#getItem` | method | 1 |
| `net/minecraft/world/clock/ClockNetworkState#totalTicks` | method | 1 |
| `net/minecraft/world/damagesource/DamageSource#getMsgId` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#cactus` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#drown` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#fall` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#fellOutOfWorld` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#generic` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#inFire` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#inWall` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#lava` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#lightningBolt` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#magic` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#onFire` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#starve` | method | 1 |
| `net/minecraft/world/damagesource/DamageSources#wither` | method | 1 |
| `net/minecraft/world/effect/MobEffectInstance#getDuration` | method | 1 |
| `net/minecraft/world/effect/MobEffects#BLINDNESS` | field | 1 |
| `net/minecraft/world/effect/MobEffects#INVISIBILITY` | field | 1 |
| `net/minecraft/world/entity/Avatar#POSES` | field | 1 |
| `net/minecraft/world/entity/Avatar#getUUID` | method | 1 |
| `net/minecraft/world/entity/Avatar#hasCustomName` | method | 1 |
| `net/minecraft/world/entity/Display#getId` | method | 1 |
| `net/minecraft/world/entity/Display#getLookAngle` | method | 2 |
| `net/minecraft/world/entity/Display#renderState` | method | 1 |
| `net/minecraft/world/entity/Display#tick` | method | 1 |
| `net/minecraft/world/entity/Display#updateRenderSubState` | method | 1 |
| `net/minecraft/world/entity/Display$BlockDisplay#getBlockState` | method | 2 |
| `net/minecraft/world/entity/Display$GenericInterpolator#get` | method | 1 |
| `net/minecraft/world/entity/Display$ItemDisplay#getItemStack` | method | 2 |
| `net/minecraft/world/entity/Display$RenderState#transformation` | method | 1 |
| `net/minecraft/world/entity/Display$TextDisplay#getText` | method | 1 |
| `net/minecraft/world/entity/Entity#DATA_CUSTOM_NAME` | field | 1 |
| `net/minecraft/world/entity/Entity#blockPosition` | method | 4 |
| `net/minecraft/world/entity/Entity#distanceToSqr` | method | 1 |
| `net/minecraft/world/entity/Entity#getBbHeight` | method | 1 |
| `net/minecraft/world/entity/Entity#getBoundingBox` | method | 3 |
| `net/minecraft/world/entity/Entity#getCustomName` | method | 1 |
| `net/minecraft/world/entity/Entity#getDeltaMovement` | method | 1 |
| `net/minecraft/world/entity/Entity#getDisplayName` | method | 2 |
| `net/minecraft/world/entity/Entity#getEyeHeight` | method | 2 |
| `net/minecraft/world/entity/Entity#getEyePosition` | method | 1 |
| `net/minecraft/world/entity/Entity#getFirstPassenger` | method | 1 |
| `net/minecraft/world/entity/Entity#getId` | method | 11 |
| `net/minecraft/world/entity/Entity#getLookAngle` | method | 1 |
| `net/minecraft/world/entity/Entity#getName` | method | 6 |
| `net/minecraft/world/entity/Entity#getPose` | method | 1 |
| `net/minecraft/world/entity/Entity#getPositionCodec` | method | 1 |
| `net/minecraft/world/entity/Entity#getUUID` | method | 3 |
| `net/minecraft/world/entity/Entity#getVehicle` | method | 1 |
| `net/minecraft/world/entity/Entity#hasCustomName` | method | 1 |
| `net/minecraft/world/entity/Entity#hashCode` | method | 4 |
| `net/minecraft/world/entity/Entity#isAlive` | method | 1 |
| `net/minecraft/world/entity/Entity#isCurrentlyGlowing` | method | 2 |
| `net/minecraft/world/entity/Entity#isRemoved` | method | 2 |
| `net/minecraft/world/entity/Entity#level` | method | 2 |
| `net/minecraft/world/entity/Entity#position` | method | 1 |
| `net/minecraft/world/entity/Entity#setId` | method | 2 |
| `net/minecraft/world/entity/Entity#setYRot` | method | 1 |
| `net/minecraft/world/entity/Entity#tickCount` | field | 5 |
| `net/minecraft/world/entity/Entity#xOld` | field | 2 |
| `net/minecraft/world/entity/Entity#yOld` | field | 2 |
| `net/minecraft/world/entity/Entity#zOld` | field | 2 |
| `net/minecraft/world/entity/EntityDimensions#eyeHeight` | method | 1 |
| `net/minecraft/world/entity/EntityEquipment#get` | method | 2 |
| `net/minecraft/world/entity/EntityEquipment#items` | field | 3 |
| `net/minecraft/world/entity/EntityEquipment#set` | method | 2 |
| `net/minecraft/world/entity/EntitySpawnReason#COMMAND` | field | 2 |
| `net/minecraft/world/entity/EquipmentSlot#CHEST` | field | 4 |
| `net/minecraft/world/entity/EquipmentSlot#FEET` | field | 3 |
| `net/minecraft/world/entity/EquipmentSlot#HEAD` | field | 6 |
| `net/minecraft/world/entity/EquipmentSlot#LEGS` | field | 4 |
| `net/minecraft/world/entity/EquipmentSlot#MAINHAND` | field | 2 |
| `net/minecraft/world/entity/EquipmentSlot#OFFHAND` | field | 2 |
| `net/minecraft/world/entity/EquipmentSlot#getId` | method | 2 |
| `net/minecraft/world/entity/EquipmentSlot#name` | method | 1 |
| `net/minecraft/world/entity/EquipmentSlot#ordinal` | method | 2 |
| `net/minecraft/world/entity/EquipmentSlot#values` | method | 3 |
| `net/minecraft/world/entity/Leashable#getLeashHolder` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#DATA_HEALTH_ID` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#blockPosition` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#equipment` | field | 2 |
| `net/minecraft/world/entity/LivingEntity#getAttributeBaseValue` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getBoundingBox` | method | 3 |
| `net/minecraft/world/entity/LivingEntity#getDisplayName` | method | 4 |
| `net/minecraft/world/entity/LivingEntity#getEffect` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getHealth` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getId` | method | 12 |
| `net/minecraft/world/entity/LivingEntity#getItemBySlot` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getName` | method | 7 |
| `net/minecraft/world/entity/LivingEntity#getUUID` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isAlive` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#isBaby` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#level` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#position` | method | 3 |
| `net/minecraft/world/entity/LivingEntity#setItemSlot` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#xo` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#yBodyRot` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#yBodyRotO` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#yHeadRot` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#yHeadRotO` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#yRotO` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#yo` | field | 1 |
| `net/minecraft/world/entity/LivingEntity#zo` | field | 1 |
| `net/minecraft/world/entity/Mob#getId` | method | 1 |
| `net/minecraft/world/entity/Mob#getItemBySlot` | method | 2 |
| `net/minecraft/world/entity/Pose#CROUCHING` | field | 1 |
| `net/minecraft/world/entity/Pose#STANDING` | field | 1 |
| `net/minecraft/world/entity/PositionMoveRotation#position` | method | 1 |
| `net/minecraft/world/entity/PositionMoveRotation#yRot` | method | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#MAX_HEALTH` | field | 1 |
| `net/minecraft/world/entity/ai/attributes/Attributes#MOVEMENT_SPEED` | field | 1 |
| `net/minecraft/world/entity/animal/cow/AbstractCow#getId` | method | 1 |
| `net/minecraft/world/entity/animal/fish/TropicalFish#getBaseColor` | method | 1 |
| `net/minecraft/world/entity/animal/fish/TropicalFish#getPattern` | method | 1 |
| `net/minecraft/world/entity/animal/fish/TropicalFish#getPatternColor` | method | 2 |
| `net/minecraft/world/entity/animal/frog/Frog#getVariant` | method | 1 |
| `net/minecraft/world/entity/animal/golem/SnowGolem#getId` | method | 1 |
| `net/minecraft/world/entity/animal/panda/Panda#getHiddenGene` | method | 1 |
| `net/minecraft/world/entity/animal/panda/Panda#getMainGene` | method | 1 |
| `net/minecraft/world/entity/animal/panda/Panda#isBrown` | method | 1 |
| `net/minecraft/world/entity/animal/panda/Panda$Gene#NORMAL` | field | 1 |
| `net/minecraft/world/entity/animal/pig/Pig#getId` | method | 2 |
| `net/minecraft/world/entity/boss/wither/WitherBoss#getId` | method | 3 |
| `net/minecraft/world/entity/boss/wither/WitherBoss#getInvulnerableTicks` | method | 2 |
| `net/minecraft/world/entity/boss/wither/WitherBoss#isPowered` | method | 1 |
| `net/minecraft/world/entity/decoration/ArmorStand#blockPosition` | method | 4 |
| `net/minecraft/world/entity/decoration/ArmorStand#distanceTo` | method | 2 |
| `net/minecraft/world/entity/decoration/ArmorStand#equipment` | field | 2 |
| `net/minecraft/world/entity/decoration/ArmorStand#getBodyPose` | method | 2 |
| `net/minecraft/world/entity/decoration/ArmorStand#getBoundingBox` | method | 2 |
| `net/minecraft/world/entity/decoration/ArmorStand#getCustomName` | method | 1 |
| `net/minecraft/world/entity/decoration/ArmorStand#getDisplayName` | method | 2 |
| `net/minecraft/world/entity/decoration/ArmorStand#getHeadPose` | method | 2 |
| `net/minecraft/world/entity/decoration/ArmorStand#getId` | method | 11 |
| `net/minecraft/world/entity/decoration/ArmorStand#getItemBySlot` | method | 2 |
| `net/minecraft/world/entity/decoration/ArmorStand#getName` | method | 23 |
| `net/minecraft/world/entity/decoration/ArmorStand#getUUID` | method | 4 |
| `net/minecraft/world/entity/decoration/ArmorStand#hasCustomName` | method | 4 |
| `net/minecraft/world/entity/decoration/ArmorStand#hashCode` | method | 1 |
| `net/minecraft/world/entity/decoration/ArmorStand#isAlive` | method | 3 |
| `net/minecraft/world/entity/decoration/ArmorStand#isInvisible` | method | 2 |
| `net/minecraft/world/entity/decoration/ArmorStand#level` | method | 1 |
| `net/minecraft/world/entity/decoration/ArmorStand#setCustomName` | method | 1 |
| `net/minecraft/world/entity/decoration/ArmorStand#tickCount` | field | 3 |
| `net/minecraft/world/entity/item/ItemEntity#getItem` | method | 8 |
| `net/minecraft/world/entity/item/ItemEntity#position` | method | 1 |
| `net/minecraft/world/entity/monster/Blaze#aiStep` | method | 1 |
| `net/minecraft/world/entity/monster/Creeper#getId` | method | 1 |
| `net/minecraft/world/entity/monster/Creeper#isIgnited` | method | 1 |
| `net/minecraft/world/entity/monster/Creeper#isPowered` | method | 2 |
| `net/minecraft/world/entity/monster/EnderMan#getCarriedBlock` | method | 1 |
| `net/minecraft/world/entity/monster/EnderMan#getVehicle` | method | 1 |
| `net/minecraft/world/entity/monster/EnderMan#teleport` | method | 1 |
| `net/minecraft/world/entity/monster/Giant#getName` | method | 1 |
| `net/minecraft/world/entity/monster/Giant#getUUID` | method | 1 |
| `net/minecraft/world/entity/monster/Giant#isInvisible` | method | 1 |
| `net/minecraft/world/entity/monster/Guardian#getId` | method | 1 |
| `net/minecraft/world/entity/monster/Shulker#getAttachFace` | method | 1 |
| `net/minecraft/world/entity/monster/Shulker#getColor` | method | 2 |
| `net/minecraft/world/entity/monster/Witch#getId` | method | 1 |
| `net/minecraft/world/entity/monster/cubemob/MagmaCube#getSize` | method | 2 |
| `net/minecraft/world/entity/monster/cubemob/MagmaCube#squish` | field | 1 |
| `net/minecraft/world/entity/monster/cubemob/Slime#getBoundingBox` | method | 1 |
| `net/minecraft/world/entity/monster/cubemob/Slime#getSize` | method | 1 |
| `net/minecraft/world/entity/monster/cubemob/Slime#position` | method | 1 |
| `net/minecraft/world/entity/monster/zombie/Zombie#getBbHeight` | method | 1 |
| `net/minecraft/world/entity/monster/zombie/Zombie#getFirstPassenger` | method | 1 |
| `net/minecraft/world/entity/monster/zombie/Zombie#getId` | method | 1 |
| `net/minecraft/world/entity/monster/zombie/Zombie#hashCode` | method | 1 |
| `net/minecraft/world/entity/monster/zombie/Zombie#isBaby` | method | 1 |
| `net/minecraft/world/entity/npc/villager/Villager#getId` | method | 1 |
| `net/minecraft/world/entity/npc/villager/Villager#getMaxHealth` | method | 1 |
| `net/minecraft/world/entity/player/Abilities#flying` | field | 1 |
| `net/minecraft/world/entity/player/Inventory#EQUIPMENT_SLOT_MAPPING` | field | 2 |
| `net/minecraft/world/entity/player/Inventory#equipment` | field | 1 |
| `net/minecraft/world/entity/player/Inventory#getNonEquipmentItems` | method | 1 |
| `net/minecraft/world/entity/player/Player#getGameProfile` | method | 6 |
| `net/minecraft/world/entity/player/Player#getInventory` | method | 1 |
| `net/minecraft/world/entity/player/Player#getItemBySlot` | method | 1 |
| `net/minecraft/world/entity/player/Player#getName` | method | 3 |
| `net/minecraft/world/entity/player/Player#getUUID` | method | 1 |
| `net/minecraft/world/entity/player/Player#getYRot` | method | 1 |
| `net/minecraft/world/entity/player/Player#isLocalPlayer` | method | 3 |
| `net/minecraft/world/entity/player/Player#tickCount` | field | 1 |
| `net/minecraft/world/entity/player/PlayerModelPart#CAPE` | field | 1 |
| `net/minecraft/world/entity/player/PlayerSkin#body` | method | 1 |
| `net/minecraft/world/entity/projectile/FishingHook#getDeltaMovement` | method | 1 |
| `net/minecraft/world/entity/projectile/FishingHook#getPlayerOwner` | method | 1 |
| `net/minecraft/world/entity/projectile/FishingHook#isInLava` | method | 2 |
| `net/minecraft/world/entity/projectile/FishingHook#isInWater` | method | 1 |
| `net/minecraft/world/entity/projectile/FishingHook#tickCount` | field | 1 |
| `net/minecraft/world/entity/projectile/arrow/Arrow#getOwner` | method | 1 |
| `net/minecraft/world/entity/projectile/hurtingprojectile/AbstractHurtingProjectile#createParticleTrail` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#containerId` | field | 3 |
| `net/minecraft/world/inventory/AbstractContainerMenu#getCarried` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#getSlot` | method | 7 |
| `net/minecraft/world/inventory/AbstractContainerMenu#hashCode` | method | 1 |
| `net/minecraft/world/inventory/AbstractContainerMenu#slots` | field | 19 |
| `net/minecraft/world/inventory/ChestMenu#addSlotListener` | method | 1 |
| `net/minecraft/world/inventory/ChestMenu#getContainer` | method | 2 |
| `net/minecraft/world/inventory/ChestMenu#removeSlotListener` | method | 1 |
| `net/minecraft/world/inventory/ChestMenu#removed` | method | 1 |
| `net/minecraft/world/inventory/ChestMenu#slots` | field | 5 |
| `net/minecraft/world/inventory/ContainerInput#CLONE` | field | 3 |
| `net/minecraft/world/inventory/ContainerInput#PICKUP` | field | 3 |
| `net/minecraft/world/inventory/ContainerInput#QUICK_MOVE` | field | 3 |
| `net/minecraft/world/inventory/ContainerInput#SWAP` | field | 3 |
| `net/minecraft/world/inventory/ContainerInput#ordinal` | method | 2 |
| `net/minecraft/world/inventory/ContainerInput#values` | method | 1 |
| `net/minecraft/world/inventory/MenuType#ANVIL` | field | 1 |
| `net/minecraft/world/inventory/MenuType#BEACON` | field | 1 |
| `net/minecraft/world/inventory/MenuType#BLAST_FURNACE` | field | 1 |
| `net/minecraft/world/inventory/MenuType#BREWING_STAND` | field | 1 |
| `net/minecraft/world/inventory/MenuType#CARTOGRAPHY_TABLE` | field | 1 |
| `net/minecraft/world/inventory/MenuType#CRAFTER_3x3` | field | 1 |
| `net/minecraft/world/inventory/MenuType#CRAFTING` | field | 1 |
| `net/minecraft/world/inventory/MenuType#ENCHANTMENT` | field | 1 |
| `net/minecraft/world/inventory/MenuType#FURNACE` | field | 1 |
| `net/minecraft/world/inventory/MenuType#GENERIC_3x3` | field | 1 |
| `net/minecraft/world/inventory/MenuType#GENERIC_9x1` | field | 1 |
| `net/minecraft/world/inventory/MenuType#GENERIC_9x2` | field | 1 |
| `net/minecraft/world/inventory/MenuType#GENERIC_9x3` | field | 1 |
| `net/minecraft/world/inventory/MenuType#GENERIC_9x4` | field | 1 |
| `net/minecraft/world/inventory/MenuType#GENERIC_9x5` | field | 1 |
| `net/minecraft/world/inventory/MenuType#GENERIC_9x6` | field | 1 |
| `net/minecraft/world/inventory/MenuType#GRINDSTONE` | field | 1 |
| `net/minecraft/world/inventory/MenuType#HOPPER` | field | 1 |
| `net/minecraft/world/inventory/MenuType#LECTERN` | field | 1 |
| `net/minecraft/world/inventory/MenuType#LOOM` | field | 1 |
| `net/minecraft/world/inventory/MenuType#MERCHANT` | field | 1 |
| `net/minecraft/world/inventory/MenuType#SHULKER_BOX` | field | 1 |
| `net/minecraft/world/inventory/MenuType#SMITHING` | field | 1 |
| `net/minecraft/world/inventory/MenuType#SMOKER` | field | 1 |
| `net/minecraft/world/inventory/MenuType#STONECUTTER` | field | 1 |
| `net/minecraft/world/inventory/MenuType#create` | method | 1 |
| `net/minecraft/world/inventory/Slot#container` | field | 7 |
| `net/minecraft/world/inventory/Slot#getContainerSlot` | method | 31 |
| `net/minecraft/world/inventory/Slot#getItem` | method | 70 |
| `net/minecraft/world/inventory/Slot#hasItem` | method | 3 |
| `net/minecraft/world/inventory/Slot#index` | field | 41 |
| `net/minecraft/world/inventory/Slot#isHighlightable` | method | 2 |
| `net/minecraft/world/inventory/Slot#x` | field | 4 |
| `net/minecraft/world/inventory/Slot#y` | field | 4 |
| `net/minecraft/world/item/DyeColor#BLACK` | field | 3 |
| `net/minecraft/world/item/DyeColor#BLUE` | field | 3 |
| `net/minecraft/world/item/DyeColor#BROWN` | field | 2 |
| `net/minecraft/world/item/DyeColor#CYAN` | field | 2 |
| `net/minecraft/world/item/DyeColor#GRAY` | field | 3 |
| `net/minecraft/world/item/DyeColor#GREEN` | field | 4 |
| `net/minecraft/world/item/DyeColor#LIGHT_BLUE` | field | 4 |
| `net/minecraft/world/item/DyeColor#LIGHT_GRAY` | field | 2 |
| `net/minecraft/world/item/DyeColor#LIME` | field | 3 |
| `net/minecraft/world/item/DyeColor#MAGENTA` | field | 3 |
| `net/minecraft/world/item/DyeColor#ORANGE` | field | 3 |
| `net/minecraft/world/item/DyeColor#PINK` | field | 2 |
| `net/minecraft/world/item/DyeColor#PURPLE` | field | 3 |
| `net/minecraft/world/item/DyeColor#RED` | field | 4 |
| `net/minecraft/world/item/DyeColor#WHITE` | field | 4 |
| `net/minecraft/world/item/DyeColor#YELLOW` | field | 3 |
| `net/minecraft/world/item/DyeColor#getId` | method | 2 |
| `net/minecraft/world/item/DyeColor#ordinal` | method | 2 |
| `net/minecraft/world/item/DyeColor#values` | method | 2 |
| `net/minecraft/world/item/Item#descriptionId` | field | 1 |
| `net/minecraft/world/item/Item#toString` | method | 1 |
| `net/minecraft/world/item/Item$TooltipContext#EMPTY` | field | 2 |
| `net/minecraft/world/item/ItemDisplayContext#GUI` | field | 1 |
| `net/minecraft/world/item/ItemStack#<init>` | method | 11 |
| `net/minecraft/world/item/ItemStack#EMPTY` | field | 7 |
| `net/minecraft/world/item/ItemStack#addToTooltip` | method | 1 |
| `net/minecraft/world/item/ItemStack#applyComponents` | method | 2 |
| `net/minecraft/world/item/ItemStack#components` | field | 1 |
| `net/minecraft/world/item/ItemStack#copy` | method | 11 |
| `net/minecraft/world/item/ItemStack#get` | method | 3 |
| `net/minecraft/world/item/ItemStack#getComponents` | method | 1 |
| `net/minecraft/world/item/ItemStack#getComponentsPatch` | method | 1 |
| `net/minecraft/world/item/ItemStack#getDisplayName` | method | 2 |
| `net/minecraft/world/item/ItemStack#getHoverName` | method | 116 |
| `net/minecraft/world/item/ItemStack#getItem` | method | 6 |
| `net/minecraft/world/item/ItemStack#getMaxStackSize` | method | 1 |
| `net/minecraft/world/item/ItemStack#getPrototype` | method | 1 |
| `net/minecraft/world/item/ItemStack#getStyledHoverName` | method | 1 |
| `net/minecraft/world/item/ItemStack#getTooltipLines` | method | 2 |
| `net/minecraft/world/item/ItemStack#has` | method | 1 |
| `net/minecraft/world/item/ItemStack#hasNonDefault` | method | 1 |
| `net/minecraft/world/item/ItemStack#hashCode` | method | 13 |
| `net/minecraft/world/item/ItemStack#immutableComponents` | method | 2 |
| `net/minecraft/world/item/ItemStack#is` | method | 19 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 8 |
| `net/minecraft/world/item/ItemStack#isEnchanted` | method | 2 |
| `net/minecraft/world/item/ItemStack#item` | field | 2 |
| `net/minecraft/world/item/ItemStack#remove` | method | 1 |
| `net/minecraft/world/item/ItemStack#set` | method | 7 |
| `net/minecraft/world/item/ItemStack#setCount` | method | 4 |
| `net/minecraft/world/item/ItemStack#typeHolder` | method | 1 |
| `net/minecraft/world/item/ItemStackTemplate#<init>` | method | 2 |
| `net/minecraft/world/item/ItemStackTemplate#fromNonEmptyStack` | method | 3 |
| `net/minecraft/world/item/Items#AIR` | field | 4 |
| `net/minecraft/world/item/Items#ARROW` | field | 3 |
| `net/minecraft/world/item/Items#BEACON` | field | 1 |
| `net/minecraft/world/item/Items#BLUE_ORCHID` | field | 1 |
| `net/minecraft/world/item/Items#BONE` | field | 2 |
| `net/minecraft/world/item/Items#BONE_MEAL` | field | 1 |
| `net/minecraft/world/item/Items#BREWING_STAND` | field | 1 |
| `net/minecraft/world/item/Items#CACTUS` | field | 1 |
| `net/minecraft/world/item/Items#CARROT` | field | 1 |
| `net/minecraft/world/item/Items#CARVED_PUMPKIN` | field | 1 |
| `net/minecraft/world/item/Items#CHAINMAIL_BOOTS` | field | 1 |
| `net/minecraft/world/item/Items#CHAINMAIL_CHESTPLATE` | field | 1 |
| `net/minecraft/world/item/Items#CHAINMAIL_HELMET` | field | 1 |
| `net/minecraft/world/item/Items#CHAINMAIL_LEGGINGS` | field | 1 |
| `net/minecraft/world/item/Items#CHEST` | field | 1 |
| `net/minecraft/world/item/Items#CLOCK` | field | 1 |
| `net/minecraft/world/item/Items#COCOA_BEANS` | field | 2 |
| `net/minecraft/world/item/Items#COD` | field | 1 |
| `net/minecraft/world/item/Items#COMPASS` | field | 1 |
| `net/minecraft/world/item/Items#DIAMOND_BOOTS` | field | 1 |
| `net/minecraft/world/item/Items#DIAMOND_CHESTPLATE` | field | 1 |
| `net/minecraft/world/item/Items#DIAMOND_HELMET` | field | 2 |
| `net/minecraft/world/item/Items#DIAMOND_LEGGINGS` | field | 1 |
| `net/minecraft/world/item/Items#DIAMOND_SWORD` | field | 2 |
| `net/minecraft/world/item/Items#DYE` | field | 1 |
| `net/minecraft/world/item/Items#EMERALD` | field | 1 |
| `net/minecraft/world/item/Items#ENCHANTED_BOOK` | field | 4 |
| `net/minecraft/world/item/Items#ENDER_EYE` | field | 1 |
| `net/minecraft/world/item/Items#ENDER_PEARL` | field | 1 |
| `net/minecraft/world/item/Items#FISHING_ROD` | field | 1 |
| `net/minecraft/world/item/Items#GOLDEN_AXE` | field | 1 |
| `net/minecraft/world/item/Items#GOLDEN_BOOTS` | field | 1 |
| `net/minecraft/world/item/Items#GOLDEN_CHESTPLATE` | field | 1 |
| `net/minecraft/world/item/Items#GOLDEN_HELMET` | field | 2 |
| `net/minecraft/world/item/Items#GOLDEN_HOE` | field | 1 |
| `net/minecraft/world/item/Items#GOLDEN_LEGGINGS` | field | 1 |
| `net/minecraft/world/item/Items#GOLDEN_PICKAXE` | field | 2 |
| `net/minecraft/world/item/Items#GOLDEN_SWORD` | field | 1 |
| `net/minecraft/world/item/Items#GOLD_NUGGET` | field | 1 |
| `net/minecraft/world/item/Items#IRON_BOOTS` | field | 1 |
| `net/minecraft/world/item/Items#IRON_CHESTPLATE` | field | 1 |
| `net/minecraft/world/item/Items#IRON_HELMET` | field | 2 |
| `net/minecraft/world/item/Items#IRON_LEGGINGS` | field | 1 |
| `net/minecraft/world/item/Items#KNOWLEDGE_BOOK` | field | 1 |
| `net/minecraft/world/item/Items#LAPIS_LAZULI` | field | 1 |
| `net/minecraft/world/item/Items#LEAD` | field | 1 |
| `net/minecraft/world/item/Items#LEATHER_BOOTS` | field | 1 |
| `net/minecraft/world/item/Items#LEATHER_CHESTPLATE` | field | 1 |
| `net/minecraft/world/item/Items#LEATHER_HELMET` | field | 2 |
| `net/minecraft/world/item/Items#LEATHER_LEGGINGS` | field | 1 |
| `net/minecraft/world/item/Items#MAP` | field | 3 |
| `net/minecraft/world/item/Items#MELON_SLICE` | field | 1 |
| `net/minecraft/world/item/Items#NAME_TAG` | field | 2 |
| `net/minecraft/world/item/Items#NETHERITE_HOE` | field | 1 |
| `net/minecraft/world/item/Items#NETHER_WART` | field | 1 |
| `net/minecraft/world/item/Items#OMINOUS_BOTTLE` | field | 1 |
| `net/minecraft/world/item/Items#PAINTING` | field | 1 |
| `net/minecraft/world/item/Items#PAPER` | field | 2 |
| `net/minecraft/world/item/Items#PLAYER_HEAD` | field | 10 |
| `net/minecraft/world/item/Items#POLAR_BEAR_SPAWN_EGG` | field | 1 |
| `net/minecraft/world/item/Items#POTATO` | field | 1 |
| `net/minecraft/world/item/Items#RED_MUSHROOM_BLOCK` | field | 1 |
| `net/minecraft/world/item/Items#ROSE_BUSH` | field | 1 |
| `net/minecraft/world/item/Items#SUGAR_CANE` | field | 1 |
| `net/minecraft/world/item/Items#SUNFLOWER` | field | 1 |
| `net/minecraft/world/item/Items#TRIDENT` | field | 1 |
| `net/minecraft/world/item/Items#WATER_BUCKET` | field | 1 |
| `net/minecraft/world/item/Items#WHEAT` | field | 1 |
| `net/minecraft/world/item/Items#WITHER_SKELETON_SKULL` | field | 1 |
| `net/minecraft/world/item/Items#WOODEN_AXE` | field | 1 |
| `net/minecraft/world/item/TooltipFlag#ADVANCED` | field | 1 |
| `net/minecraft/world/item/TooltipFlag#NORMAL` | field | 2 |
| `net/minecraft/world/item/component/CustomData#copyTag` | method | 3 |
| `net/minecraft/world/item/component/CustomData#of` | method | 4 |
| `net/minecraft/world/item/component/DyedItemColor#<init>` | method | 1 |
| `net/minecraft/world/item/component/DyedItemColor#rgb` | method | 1 |
| `net/minecraft/world/item/component/ItemLore#<init>` | method | 2 |
| `net/minecraft/world/item/component/ItemLore#lines` | method | 1 |
| `net/minecraft/world/item/component/ResolvableProfile#createResolved` | method | 3 |
| `net/minecraft/world/item/component/ResolvableProfile#name` | method | 1 |
| `net/minecraft/world/item/component/ResolvableProfile#partialProfile` | method | 3 |
| `net/minecraft/world/item/component/TooltipDisplay#<init>` | method | 1 |
| `net/minecraft/world/item/component/TooltipDisplay#DEFAULT` | field | 1 |
| `net/minecraft/world/item/component/TooltipDisplay#hiddenComponents` | method | 1 |
| `net/minecraft/world/item/component/TooltipDisplay#withHidden` | method | 1 |
| `net/minecraft/world/item/component/TooltipProvider#addToTooltip` | method | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments#EMPTY` | field | 1 |
| `net/minecraft/world/level/ClipContext#<init>` | method | 1 |
| `net/minecraft/world/level/ClipContext$Block#COLLIDER` | field | 1 |
| `net/minecraft/world/level/ClipContext$Fluid#NONE` | field | 1 |
| `net/minecraft/world/level/GameType#SPECTATOR` | field | 1 |
| `net/minecraft/world/level/Level#addParticle` | method | 2 |
| `net/minecraft/world/level/Level#getEntity` | method | 1 |
| `net/minecraft/world/level/block/BaseFireBlock#animateTick` | method | 1 |
| `net/minecraft/world/level/block/Block#asItem` | method | 5 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 2 |
| `net/minecraft/world/level/block/Block#getName` | method | 1 |
| `net/minecraft/world/level/block/Block#getStateDefinition` | method | 1 |
| `net/minecraft/world/level/block/Block#toString` | method | 2 |
| `net/minecraft/world/level/block/Blocks#ACACIA_LEAVES` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ACACIA_LOG` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ACACIA_WOOD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 6 |
| `net/minecraft/world/level/block/Blocks#ALLIUM` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ANDESITE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#AZALEA_LEAVES` | field | 1 |
| `net/minecraft/world/level/block/Blocks#AZURE_BLUET` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BARRIER` | field | 3 |
| `net/minecraft/world/level/block/Blocks#BEACON` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BEDROCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BIRCH_LEAVES` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BIRCH_LOG` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BIRCH_WOOD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BLUE_ORCHID` | field | 1 |
| `net/minecraft/world/level/block/Blocks#BROWN_MUSHROOM` | field | 3 |
| `net/minecraft/world/level/block/Blocks#CACTUS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CARROTS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CARVED_PUMPKIN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CHERRY_LOG` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CHERRY_WOOD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CHEST` | field | 2 |
| `net/minecraft/world/level/block/Blocks#CHISELED_STONE_BRICKS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CLAY` | field | 4 |
| `net/minecraft/world/level/block/Blocks#COAL_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#COAL_ORE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#COBBLESTONE` | field | 3 |
| `net/minecraft/world/level/block/Blocks#COBBLESTONE_SLAB` | field | 1 |
| `net/minecraft/world/level/block/Blocks#COBBLESTONE_STAIRS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#COCOA` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CRACKED_STONE_BRICKS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CRAFTING_TABLE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CRIMSON_HYPHAE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CRIMSON_STEM` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DANDELION` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DARK_OAK_LEAVES` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DARK_OAK_LOG` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DARK_OAK_WOOD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DARK_PRISMARINE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DEAD_BUSH` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DIAMOND_BLOCK` | field | 2 |
| `net/minecraft/world/level/block/Blocks#DIAMOND_ORE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DIORITE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DRAGON_EGG` | field | 1 |
| `net/minecraft/world/level/block/Blocks#DYED_TERRACOTTA` | field | 1 |
| `net/minecraft/world/level/block/Blocks#EMERALD_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#EMERALD_ORE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ENCHANTING_TABLE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ENDER_CHEST` | field | 1 |
| `net/minecraft/world/level/block/Blocks#END_PORTAL_FRAME` | field | 1 |
| `net/minecraft/world/level/block/Blocks#END_STONE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#FARMLAND` | field | 1 |
| `net/minecraft/world/level/block/Blocks#FERN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GLASS_PANE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GLOWSTONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GOLD_BLOCK` | field | 2 |
| `net/minecraft/world/level/block/Blocks#GOLD_ORE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GRANITE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GRASS_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GRAVEL` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ICE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#INFESTED_COBBLESTONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#INFESTED_STONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#IRON_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#IRON_ORE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#JUNGLE_LEAVES` | field | 1 |
| `net/minecraft/world/level/block/Blocks#JUNGLE_LOG` | field | 1 |
| `net/minecraft/world/level/block/Blocks#JUNGLE_WOOD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LAPIS_BLOCK` | field | 3 |
| `net/minecraft/world/level/block/Blocks#LAPIS_ORE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LARGE_FERN` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LAVA` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LEVER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LILAC` | field | 1 |
| `net/minecraft/world/level/block/Blocks#MANGROVE_LEAVES` | field | 1 |
| `net/minecraft/world/level/block/Blocks#MANGROVE_LOG` | field | 1 |
| `net/minecraft/world/level/block/Blocks#MANGROVE_WOOD` | field | 2 |
| `net/minecraft/world/level/block/Blocks#MELON` | field | 2 |
| `net/minecraft/world/level/block/Blocks#MOSSY_STONE_BRICKS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#MYCELIUM` | field | 1 |
| `net/minecraft/world/level/block/Blocks#NETHERRACK` | field | 2 |
| `net/minecraft/world/level/block/Blocks#NETHER_QUARTZ_ORE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#NETHER_WART` | field | 1 |
| `net/minecraft/world/level/block/Blocks#OAK_BUTTON` | field | 1 |
| `net/minecraft/world/level/block/Blocks#OAK_LEAVES` | field | 2 |
| `net/minecraft/world/level/block/Blocks#OAK_LOG` | field | 1 |
| `net/minecraft/world/level/block/Blocks#OAK_WOOD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#OBSIDIAN` | field | 2 |
| `net/minecraft/world/level/block/Blocks#ORANGE_TULIP` | field | 1 |
| `net/minecraft/world/level/block/Blocks#OXEYE_DAISY` | field | 1 |
| `net/minecraft/world/level/block/Blocks#PACKED_ICE` | field | 4 |
| `net/minecraft/world/level/block/Blocks#PALE_OAK_LOG` | field | 1 |
| `net/minecraft/world/level/block/Blocks#PALE_OAK_WOOD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#PEONY` | field | 1 |
| `net/minecraft/world/level/block/Blocks#PINK_TULIP` | field | 1 |
| `net/minecraft/world/level/block/Blocks#PLAYER_HEAD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#POLISHED_DIORITE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#POPPY` | field | 1 |
| `net/minecraft/world/level/block/Blocks#POTATOES` | field | 1 |
| `net/minecraft/world/level/block/Blocks#PRISMARINE` | field | 2 |
| `net/minecraft/world/level/block/Blocks#PRISMARINE_BRICKS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#QUARTZ_BLOCK` | field | 1 |
| `net/minecraft/world/level/block/Blocks#QUARTZ_STAIRS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#REDSTONE_BLOCK` | field | 2 |
| `net/minecraft/world/level/block/Blocks#REDSTONE_LAMP` | field | 1 |
| `net/minecraft/world/level/block/Blocks#REDSTONE_ORE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#RED_MUSHROOM` | field | 3 |
| `net/minecraft/world/level/block/Blocks#RED_SAND` | field | 1 |
| `net/minecraft/world/level/block/Blocks#RED_SANDSTONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#RED_TULIP` | field | 1 |
| `net/minecraft/world/level/block/Blocks#ROSE_BUSH` | field | 2 |
| `net/minecraft/world/level/block/Blocks#SAND` | field | 2 |
| `net/minecraft/world/level/block/Blocks#SANDSTONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SANDSTONE_STAIRS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SHORT_GRASS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SMOOTH_RED_SANDSTONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SPONGE` | field | 3 |
| `net/minecraft/world/level/block/Blocks#SPRUCE_FENCE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SPRUCE_LEAVES` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SPRUCE_LOG` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SPRUCE_WOOD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STAINED_GLASS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STAINED_GLASS_PANE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STONE` | field | 3 |
| `net/minecraft/world/level/block/Blocks#STONE_BRICKS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STRIPPED_BIRCH_WOOD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STRIPPED_MANGROVE_WOOD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#STRIPPED_SPRUCE_WOOD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SUGAR_CANE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SUNFLOWER` | field | 2 |
| `net/minecraft/world/level/block/Blocks#TALL_GRASS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#TERRACOTTA` | field | 1 |
| `net/minecraft/world/level/block/Blocks#TRAPPED_CHEST` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WARPED_HYPHAE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WARPED_STEM` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WATER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WHEAT` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WHITE_TULIP` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WOOL` | field | 1 |
| `net/minecraft/world/level/block/ButtonBlock#POWERED` | field | 1 |
| `net/minecraft/world/level/block/ColorCollection#black` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#blue` | method | 1 |
| `net/minecraft/world/level/block/ColorCollection#brown` | method | 1 |
| `net/minecraft/world/level/block/ColorCollection#cyan` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#gray` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#green` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#lightBlue` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#lightGray` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#lime` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#magenta` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#orange` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#pink` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#purple` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#red` | method | 2 |
| `net/minecraft/world/level/block/ColorCollection#white` | method | 1 |
| `net/minecraft/world/level/block/ColorCollection#yellow` | method | 2 |
| `net/minecraft/world/level/block/StainedGlassBlock#getColor` | method | 1 |
| `net/minecraft/world/level/block/StainedGlassPaneBlock#getColor` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntity#getBlockPos` | method | 1 |
| `net/minecraft/world/level/block/entity/ChestBlockEntity#getOpenNess` | method | 1 |
| `net/minecraft/world/level/block/entity/SignText#getMessages` | method | 1 |
| `net/minecraft/world/level/block/entity/SkullBlockEntity#getOwnerProfile` | method | 1 |
| `net/minecraft/world/level/block/entity/TickingBlockEntity#getPos` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 21 |
| `net/minecraft/world/level/block/state/BlockState#getShape` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getValue` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 1 |
| `net/minecraft/world/level/block/state/StateDefinition#getProperties` | method | 1 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#LIT` | field | 1 |
| `net/minecraft/world/level/block/state/properties/Property#getName` | method | 1 |
| `net/minecraft/world/level/dimension/DimensionType#defaultClock` | method | 1 |
| `net/minecraft/world/level/entity/TransientEntitySectionManager$Callback#onRemove` | method | 1 |
| `net/minecraft/world/level/material/FluidState#getType` | method | 1 |
| `net/minecraft/world/level/material/Fluids#FLOWING_LAVA` | field | 1 |
| `net/minecraft/world/level/material/Fluids#LAVA` | field | 1 |
| `net/minecraft/world/level/material/Fluids#WATER` | field | 1 |
| `net/minecraft/world/level/material/FogType#WATER` | field | 1 |
| `net/minecraft/world/phys/AABB#<init>` | method | 17 |
| `net/minecraft/world/phys/AABB#contains` | method | 2 |
| `net/minecraft/world/phys/AABB#expandTowards` | method | 1 |
| `net/minecraft/world/phys/AABB#hashCode` | method | 4 |
| `net/minecraft/world/phys/AABB#inflate` | method | 5 |
| `net/minecraft/world/phys/AABB#maxX` | field | 7 |
| `net/minecraft/world/phys/AABB#maxY` | field | 7 |
| `net/minecraft/world/phys/AABB#maxZ` | field | 7 |
| `net/minecraft/world/phys/AABB#minX` | field | 7 |
| `net/minecraft/world/phys/AABB#minY` | field | 7 |
| `net/minecraft/world/phys/AABB#minZ` | field | 7 |
| `net/minecraft/world/phys/AABB#move` | method | 4 |
| `net/minecraft/world/phys/BlockHitResult#getBlockPos` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#getLocation` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#miss` | field | 2 |
| `net/minecraft/world/phys/EntityHitResult#getEntity` | method | 1 |
| `net/minecraft/world/phys/HitResult#getLocation` | method | 1 |
| `net/minecraft/world/phys/HitResult#getType` | method | 2 |
| `net/minecraft/world/phys/HitResult$Type#BLOCK` | field | 3 |
| `net/minecraft/world/phys/HitResult$Type#ENTITY` | field | 1 |
| `net/minecraft/world/phys/HitResult$Type#MISS` | field | 1 |
| `net/minecraft/world/phys/HitResult$Type#ordinal` | method | 2 |
| `net/minecraft/world/phys/HitResult$Type#values` | method | 1 |
| `net/minecraft/world/phys/Vec3#<init>` | method | 3 |
| `net/minecraft/world/phys/Vec3#ZERO` | field | 6 |
| `net/minecraft/world/phys/Vec3#hashCode` | method | 2 |
| `net/minecraft/world/phys/Vec3#x` | field | 12 |
| `net/minecraft/world/phys/Vec3#y` | field | 16 |
| `net/minecraft/world/phys/Vec3#z` | field | 11 |
| `net/minecraft/world/phys/shapes/VoxelShape#isEmpty` | method | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#toAabbs` | method | 1 |
| `net/minecraft/world/scores/DisplaySlot#SIDEBAR` | field | 1 |
| `net/minecraft/world/scores/Objective#getDisplayName` | method | 2 |
| `net/minecraft/world/scores/PlayerScoreEntry#owner` | method | 1 |
| `net/minecraft/world/scores/PlayerScoreEntry#value` | method | 1 |
| `net/minecraft/world/scores/PlayerTeam#<init>` | method | 1 |
| `net/minecraft/world/scores/PlayerTeam#getName` | method | 1 |
| `net/minecraft/world/scores/PlayerTeam#getPlayerPrefix` | method | 1 |
| `net/minecraft/world/scores/PlayerTeam#getPlayerSuffix` | method | 1 |
| `net/minecraft/world/scores/Scoreboard#<init>` | method | 1 |
| `net/minecraft/world/scores/Scoreboard#getDisplayObjective` | method | 1 |
| `net/minecraft/world/scores/Scoreboard#getPlayersTeam` | method | 1 |
| `net/minecraft/world/scores/Scoreboard#listPlayerScores` | method | 1 |
| `net/minecraft/world/scores/Team$Visibility#NEVER` | field | 1 |
| `net/minecraft/world/scores/criteria/ObjectiveCriteria$RenderType#INTEGER` | field | 1 |

</details>

