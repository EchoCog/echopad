import React, { useCallback, useContext, type FormEvent } from "react";
import { PromptContext } from "../contexts/PromptContext";

import iconArrowUpward from "../../icons/arrow_upward.svg";
import {
  conversationPromptInput,
  conversationPromptInput__button,
  conversationPromptInput__controls,
  conversationPromptInput__controls__track,
  conversationPromptInput__textarea,
} from "./ConversationPromptInput.module.css";

export function ConversationPromptInput() {
  const {
    currentPrompt,
    isCurrentPromptEmpty,
    setCurrentPrompt,
    setSubmittedPrompt,
  } = useContext(PromptContext);

  const onSubmit = useCallback(
    function (event: FormEvent<HTMLFormElement>) {
      event.preventDefault();

      if (currentPrompt.trim() === "") {
        setSubmittedPrompt(null);
      } else {
        setSubmittedPrompt(currentPrompt);
      }
    },
    [currentPrompt, setSubmittedPrompt],
  );

  const onTextareaInput = useCallback(
    function (event: FormEvent<HTMLInputElement>) {
      setCurrentPrompt(event.currentTarget.value);
    },
    [setCurrentPrompt],
  );

  return (
    <form className={conversationPromptInput} onSubmit={onSubmit}>
      <input
        autoFocus
        className={conversationPromptInput__textarea}
        placeholder="Type your prompt here..."
        value={currentPrompt}
        onInput={onTextareaInput}
      />
      <div className={conversationPromptInput__controls}>
        <div className={conversationPromptInput__controls__track} />
        <div className={conversationPromptInput__controls__track}>
          <button
            className={conversationPromptInput__button}
            disabled={isCurrentPromptEmpty}
          >
            <img src={iconArrowUpward} alt="Send" />
          </button>
        </div>
      </div>
    </form>
  );
}
