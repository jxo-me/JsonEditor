<script setup lang="ts">
import {ref} from 'vue';
import { faCopy, faTrashCan } from '@fortawesome/free-regular-svg-icons'
import copyToClipboard from './utils/copyToClipboard'

import type {QueryLanguageId} from 'vue3-ts-jsoneditor'

const jsonData = ref('');

const queryLanguages = ref<QueryLanguageId[]>(['javascript', 'lodash', 'jmespath']);

const onError = (error: any) => {
  //
}

const onFocus = () => {
  //
}

const onBlur = () => {
  //
}
const handleCopy = () => {
  console.log('Custom copy button clicked')
  const contents = JSON.stringify(jsonData.value, null, 2)
  console.log('contents:', contents)
  copyToClipboard(contents)
}

const handleTrash = () => {
  console.log('Custom trash button clicked')
  console.log('value:', jsonData.value)
  // jsonData.value = {}
}

const handleRenderMenu = (items: any[], mode: any) => {
  const separator = {
    separator: true
  }
  console.log('mode:', mode)
  console.log('items:', items)
  const customCopyButton = {
    onClick: handleCopy,
    icon: faCopy,
    title: 'Copy document to clipboard',
    className: 'custom-copy-button'
  }
  const customTrashButton = {
    onClick: handleTrash,
    icon: faTrashCan,
    title: 'clear content',
    className: 'custom-copy-button'
  }
  const space = {
    space: true
  }
  const itemsWithoutSpace = items.slice(0, items.length - 1)
  return itemsWithoutSpace.concat([separator, customCopyButton, customTrashButton, space])
}
</script>

<template>
  <json-editor
      mode="text"
      :queryLanguagesIds="queryLanguages"
      v-model:json="jsonData"
      @error="onError"
      @focus="onFocus"
      @blur="onBlur"
      class="awesome-json-editor vue-ts-json-editor--max-box"
      :on-render-menu="handleRenderMenu"
  />
</template>

<style scoped>
.awesome-json-editor {
  /* define a custom theme color */
  /* over all fonts, sizes, and colors */
  --jse-font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen-Sans, Ubuntu,
  Cantarell, 'Helvetica Neue', sans-serif;
  /* "consolas" for Windows, "menlo" for Mac with fallback to "monaco", 'Ubuntu Mono' for Ubuntu */
  /* (at Mac this font looks too large at 14px, but 13px is too small for the font on Windows) */
  --jse-font-family-mono: consolas, menlo, monaco, 'Ubuntu Mono', 'source-code-pro', monospace;
  --jse-font-size-mono: 14px;
  --jse-font-size: 16px;
  --jse-font-size-text-mode-search: 80%;
  --jse-line-height: calc(1em + 4px);
  --jse-indent-size: calc(1em + 4px);
  --jse-color-picker-button-size: 1em;
  --jse-padding: 10px;
  --jse-theme-color: #3c3d3e;
  --jse-theme-color-highlight: #6c6c68;
  --jse-background-color: #fff;
  --jse-text-color: #4d4d4d;
  --jse-text-color-inverse: #fff;
  --jse-error-color: #ee5341;
  --jse-warning-color: #fdc539;

  /* main, menu, modal */
  --jse-main-border: 1px solid #d7d7d7;
  --jse-menu-color: var(--jse-text-color-inverse);
  --jse-menu-button-size: 32px;
  --jse-modal-background: #f5f5f5;
  --jse-modal-overlay-background: rgba(0, 0, 0, 0.3);
  --jse-modal-code-background: rgba(0, 0, 0, 0.05);

  /* panels: navigation bar, gutter, search box */
  --jse-panel-background: #ebebeb;
  --jse-panel-color: var(--jse-text-color);
  --jse-panel-color-readonly: #b2b2b2;
  --jse-panel-border: var(--jse-main-border);
  --jse-panel-button-color: inherit;
  --jse-panel-button-background: transparent;
  --jse-panel-button-color-highlight: var(--jse-text-color);
  --jse-panel-button-background-highlight: #e0e0e0;

  /* navigation-bar */
  --jse-navigation-bar-color: #656565;
  --jse-navigation-bar-background: var(--jse-background-color);
  --jse-navigation-bar-background-highlight: #e5e5e5;
  --jse-navigation-bar-dropdown-color: #656565;

  /* context menu */
  --jse-context-menu-background: #656565;
  --jse-context-menu-background-highlight: #7a7a7a;
  --jse-context-menu-color: var(--jse-text-color-inverse);
  --jse-context-menu-color-disabled: #9d9d9d;
  --jse-context-menu-separator-color: #7a7a7a;
  --jse-context-menu-button-background: var(--jse-context-menu-background);
  --jse-context-menu-button-background-highlight: var(--jse-context-menu-background-highlight);
  --jse-context-menu-button-color: var(--jse-context-menu-color);
  --jse-context-menu-button-size: calc(1em + 4px);
  --jse-context-menu-tip-background: rgba(255, 255, 255, 0.2);
  --jse-context-menu-tip-color: inherit;

  /* contents: json key and values */
  --jse-key-color: #1a1a1a;
  --jse-value-color: #1a1a1a;
  --jse-value-color-number: #ee422e;
  --jse-value-color-boolean: #ff8c00;
  --jse-value-color-null: #004ed0;
  --jse-value-color-string: #008000;
  --jse-value-color-url: #008000;
  --jse-delimiter-color: rgba(0, 0, 0, 0.38);
  --jse-edit-outline: 2px solid #656565;

  /* contents: selected or hovered */
  --jse-hover-background-color: rgba(0, 0, 0, 0.06);
  --jse-selection-background-color: #d3d3d3;
  --jse-selection-background-light-color: #e8e8e8;

  /* contents: section of collapsed items in an array */
  --jse-collapsed-items-background-color: #f5f5f5;
  --jse-collapsed-items-selected-background-color: #c2c2c2;
  --jse-collapsed-items-link-color: rgba(0, 0, 0, 0.38);
  --jse-collapsed-items-link-color-highlight: #ee5341;

  /* contents: highlighting of search matches */
  --jse-search-match-color: #ffe665;
  --jse-search-match-outline: 1px solid #ffd700;
  --jse-search-match-active-color: #ffd700;
  --jse-search-match-active-outline: 1px solid #e1be00;

  /* contents: inline tags inside the JSON document */
  --jse-tag-background: rgba(0, 0, 0, 0.2);
  --jse-tag-color: var(--jse-text-color-inverse);

  /* controls in modals: inputs, buttons, and `a` */
  --jse-controls-box-shadow: 0 2px 6px 0 rgba(0, 0, 0, 0.24);
  --jse-input-background: var(--jse-background-color);
  --jse-input-background-readonly: transparent;
  --jse-input-border: 1px solid #d8dbdf;
  --jse-input-border-focus: 1px solid var(--jse-theme-color);
  --jse-input-radius: 3px;
  --jse-button-background: #e0e0e0;
  --jse-button-background-highlight: #e7e7e7;
  --jse-button-color: var(--jse-text-color);
  --jse-button-primary-background: var(--jse-theme-color);
  --jse-button-primary-background-highlight: var(--jse-theme-color-highlight);
  --jse-button-primary-background-disabled: #9d9d9d;
  --jse-button-primary-color: var(--jse-text-color-inverse);
  --jse-a-color: #156fc5;
  --jse-a-color-highlight: #0f508d;

  /* messages */
  --jse-message-error-background: var(--jse-error-color);
  --jse-message-error-color: var(--jse-text-color-inverse);
  --jse-message-warning-background: #ffde5c;
  --jse-message-warning-color: var(--jse-text-color);
  --jse-message-success-background: #9ac45d;
  --jse-message-success-color: var(--jse-text-color-inverse);
  --jse-message-info-background: #9d9d9d;
  --jse-message-info-color: var(--jse-text-color-inverse);
  --jse-message-action-background: rgba(255, 255, 255, 0.2);
  --jse-message-action-background-highlight: rgba(255, 255, 255, 0.3);

  /* svelte-select */
  --itemIsActiveBG: #3883fa;
  --border: 1px solid #d8dbdf;
  --borderRadius: 3px;
  --background: #fff;

  /* color picker */
  --jse-color-picker-background: var(--jse-panel-background);
  --jse-color-picker-border-box-shadow: #cbcbcb 0 0 0 1px;
}
</style>
