import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import JsonEditor from 'vue3-ts-jsoneditor';

const app = createApp(App);

app.use(JsonEditor, {
    componentName: 'JsonEditor', // Default: 'JsonEditor',
    options: {
        /**
         *
         * SET GLOBAL OPTIONS
         *
         * */
        readOnly: false, // boolean;
        // indentation: 2, // number | string;
        // tabSize: 4, // number;
        mode: "text",
        mainMenuBar: true,
        navigationBar: true,
        statusBar: true,
        escapeControlCharacters: true,
        escapeUnicodeCharacters: false,
        // validator?: Validator;
        // queryLanguagesIds?: QueryLanguageId[];
        // queryLanguageId?: QueryLanguageId;
        // onRenderValue?: OnRenderValue;
        // onClassName?: OnClassName;
        // onRenderMenu?: OnRenderMenu;
        // height: "100%", // string | number;
        fullWidthButton: false,
        darkTheme: false,
    }
}).mount('#app')
