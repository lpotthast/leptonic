/*
 * This file is part of the leptos-tiptap crate.
 * If you see this file as part of your build.rs output, do not modify it.
 */

window._leptosTiptapEditors = new Map();

function _setEditor(id, editor, onSelection) {
  window._leptosTiptapEditors.set(id, { editor, onSelection })
}

function _forgetEditor(id) {
  window._leptosTiptapEditors.set(id, undefined)
}

function _getEditor(id) {
  return window._leptosTiptapEditors.get(id)
}

export function create(id, content, editable, onChange, onSelection) {
  var myElem = document.getElementById(id);
  if (myElem == null) {
    console.error('Can not create Tiptap instance on element with id "' + id + '", as element could not be found. You may have executed this function when the element was not yet mounted to the DOM.');
  }

  var editor = new window.TipTap.Editor({
    element: myElem,
    editable: editable,
    extensions: [
      window.TipTapStarterKit.StarterKit,
      window.TipTapTextAlign.TextAlign.configure({
        types: ['heading', 'paragraph'],
      }),
      window.TipTapHighlight.Highlight,
      window.TipTapImage.Image,
      window.TipTapLink.Link,
      window.TipTapYoutube.Youtube
    ],
    injectCSS: false,
    content: content,
    onUpdate: ({ editor }) => {
      const html = editor.getHTML();
      onChange(html);
    },
    onSelectionUpdate: ({ editor }) => {
      onSelection(_getSelectionState(editor));
    },
  });

  _setEditor(id, editor, onSelection);
}

export function destroy(id) {
  const editorWindow = _getEditor(id);
  if (editorWindow && editorWindow.editor) {
    editorWindow.editor.destroy();
    _forgetEditor(id);
  }
}

export function getHTML(id) {
  const { editor, _onSelection } = _getEditor(id);
  return editor.getHTML();
}

export function isEditable(id) {
  const { editor, _onSelection } = _getEditor(id);
  return editor.isEditable
}

export function setEditable(id, editable) {
  const { editor, _onSelection } = _getEditor(id);
  editor.setEditable(editable);
}

export function toggleHeading(id, level) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().toggleHeading({ level: level }).run();
  onSelection(_getSelectionState(editor));
}

export function setParagraph(id) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().setParagraph().run();
  onSelection(_getSelectionState(editor));
}

export function toggleBold(id) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().toggleBold().run();
  onSelection(_getSelectionState(editor));
}

export function toggleItalic(id) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().toggleItalic().run();
  onSelection(_getSelectionState(editor));
}

export function toggleStrike(id) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().toggleStrike().run();
  onSelection(_getSelectionState(editor));
}

export function toggleBlockquote(id) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().toggleBlockquote().run();
  onSelection(_getSelectionState(editor));
}

export function toggleHighlight(id) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().toggleHighlight().run();
  onSelection(_getSelectionState(editor));
}

export function toggleBulletList(id) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().toggleBulletList().run();
  onSelection(_getSelectionState(editor));
}

export function toggleOrderedList(id) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().toggleOrderedList().run();
  onSelection(_getSelectionState(editor));
}

export function setTextAlignLeft(id) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().setTextAlign('left').run();
  onSelection(_getSelectionState(editor));
}

export function setTextAlignCenter(id) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().setTextAlign('center').run();
  onSelection(_getSelectionState(editor));
}

export function setTextAlignRight(id) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().setTextAlign('right').run();
  onSelection(_getSelectionState(editor));
}

export function setTextAlignJustify(id) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().setTextAlign('justify').run();
  onSelection(_getSelectionState(editor));
}

export function setImage(id, src, alt, title) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().setImage({ src: src, alt: alt, title: title }).run();
  onSelection(_getSelectionState(editor));
}

export function setLink(id, href, target, rel) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().setLink({ href: href, target: target, rel: rel }).run();
  onSelection(_getSelectionState(editor));
}

export function toggleLink(id, href, target, rel) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().toggleLink({ href: href, target: target, rel: rel }).run();
  onSelection(_getSelectionState(editor));
}

export function unsetLink(id, href, target, rel) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().unsetLink({ href: href, target: target, rel: rel }).run();
  onSelection(_getSelectionState(editor));
}

export function setYoutubeVideo(id, src, start, width, height) {
  const { editor, onSelection } = _getEditor(id);
  editor.chain().focus().setYoutubeVideo({ src: src, start: start, width: width, height: height }).run();
  onSelection(_getSelectionState(editor));
}

export function getEditorState(id) {
  const { editor, _onSelection } = _getEditor(id);
  return _getEditorState(editor);
}

export function getSelectionState(id) {
  const { editor, _onSelection } = _getEditor(id);
  return _getSelectionState(editor);
}

function _getEditorState(editor) {
  return {
    editable: editor.isEditable,
    selection: _getSelectionState(editor)
  }
}

function _getSelectionState(editor) {
  return {
    h1: editor.isActive('heading', { level: 1 }),
    h2: editor.isActive('heading', { level: 2 }),
    h3: editor.isActive('heading', { level: 3 }),
    h4: editor.isActive('heading', { level: 4 }),
    h5: editor.isActive('heading', { level: 5 }),
    h6: editor.isActive('heading', { level: 6 }),
    paragraph: editor.isActive('paragraph'),
    bold: editor.isActive('bold'),
    italic: editor.isActive('italic'),
    strike: editor.isActive('strike'),
    blockquote: editor.isActive('blockquote'),
    highlight: editor.isActive('highlight'),
    bullet_list: editor.isActive('bulletList'),
    ordered_list: editor.isActive('orderedList'),
    align_left: editor.isActive({ textAlign: 'left' }),
    align_center: editor.isActive({ textAlign: 'center' }),
    align_right: editor.isActive({ textAlign: 'right' }),
    align_justify: editor.isActive({ textAlign: 'justify' }),
    link: editor.isActive('link'),
    youtube: editor.isActive('youtube'),
  }
}