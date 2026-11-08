// Not ported yet: the Java @ModifyReturnValue(RETURN) on CommandDispatcher.getCompletionSuggestions (MixinCommandDispatcher.getCompletionSuggestionsWIthExisting).
// Why it's off: No Horizon function mapped for CommandDispatcher.getCompletionSuggestions (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: No Horizon function known for CommandDispatcher.getCompletionSuggestions: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CommandDispatcher.getCompletionSuggestions -> (not mapped yet)
//   String.toLowerCase -> (not mapped yet)
//
// Decompiled Java, at/hannibal2/skyhanni/mixins/transformers/MixinCommandDispatcher.java:42:
// | @ModifyReturnValue(method = "getCompletionSuggestions(Lcom/mojang/brigadier/ParseResults;I)Ljava/util/concurrent/CompletableFuture;", at = @At("RETURN"))
// | public CompletableFuture<Suggestions> getCompletionSuggestionsWIthExisting(
// |    CompletableFuture<Suggestions> original, @Local(ordinal = 1) int start, @Local(ordinal = 0) String fullInput, @Local(ordinal = 1) String beforeCursor
// | ) {
// |    return beforeCursor.contains(" ") ? original : original.thenApply(suggestions -> {
// |       ArrayList<String> suggestionList = new ArrayList<>(suggestions.getList().stream().map(Suggestion::getText).toList());
// |       SuggestionsBuilder newSuggestions = this.buildFromEvent(start, fullInput, beforeCursor, suggestionList);
// |       return (Suggestions)(newSuggestions == null ? suggestions : newSuggestions.build());
// |    });
// | }
