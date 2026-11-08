// Not ported yet: the Java @Inject(INVOKE) on CommandDispatcher.getCompletionSuggestions (MixinCommandDispatcher.getCompletionSuggestions).
// Why it's off: No Horizon function mapped for CommandDispatcher.getCompletionSuggestions (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for CommandDispatcher.getCompletionSuggestions: run `match`.
// Note: The call it hooks (String.toLowerCase) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CommandDispatcher.getCompletionSuggestions -> (not mapped yet)
//   String.toLowerCase -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinCommandDispatcher.java:21:
// | @Inject(
// |    method = "getCompletionSuggestions(Lcom/mojang/brigadier/ParseResults;I)Ljava/util/concurrent/CompletableFuture;",
// |    at = @At(value = "INVOKE", target = "Ljava/lang/String;toLowerCase(Ljava/util/Locale;)Ljava/lang/String;"),
// |    cancellable = true
// | )
// | public void getCompletionSuggestions(
// |    ParseResults<S> parse,
// |    int cursor,
// |    CallbackInfoReturnable<CompletableFuture<Suggestions>> cir,
// |    @Local(ordinal = 1) int start,
// |    @Local(ordinal = 0) String fullInput,
// |    @Local(ordinal = 1) String beforeCursor
// | ) {
// |    if (beforeCursor.contains(" ")) {
// |       SuggestionsBuilder suggestionsBuilder = this.buildFromEvent(start, fullInput, beforeCursor, new ArrayList<>());
// |       if (suggestionsBuilder != null) {
// |          cir.setReturnValue(suggestionsBuilder.buildFuture());
// |       }
// |    }
// | }
