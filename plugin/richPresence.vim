if !exists("s:presenceJobId")
    let s:presenceJobId = 0
endif

let richPresence#execdir = resolve(expand('<sfile>:p:h') . '/..')
let s:bin = richPresence#execdir . '/target/release/nvimsence-rs'

function! s:connect()
    let id = richPresence#init()

    call s:loadDefaultConfig()

    if 0 == id
        echoerr "Failed to start Rich Presence"
    elseif -1 == id
        echoerr "Rich Presence binary cannot be executed"
    else
        let s:presenceJobId = id

        command! UpdateRichPresence silent! call rpcnotify(s:presenceJobId, 'update')
        command! ReconnectRichPresence call s:RestartWrapper(function('rpcnotify'), s:presenceJobId, 'reconnect')
        command! DisconnectRichPresence call s:RestartWrapper(function('rpcnotify'), s:presenceJobId, 'disconnect')
        command! RestartRichPresence call s:restartPlugin()
    endif
endfunction

function! s:loadDefaultConfig()
    let g:nvimsence_details = "{project}/{filename}"
    let g:nvimsence_something = "..."
    let g:nvimsence_nothing = "..."
endfunction

function! s:RestartWrapper(callback, ...)
    try
        silent! call a:callback(a:1, a:2)
    catch
        silent! call s:restartPlugin()
        call a:callback(a:1, a:2)
    endtry
endfunction

function! s:restartPlugin()
    call jobstop(s:presenceJobId)
    let s:presenceJobId = 0
    let s:presenceJobId = richPresence#init()
endfunction

function! richPresence#init()
    if s:presenceJobId == 0
        let jobId = jobstart([s:bin], { 'rpc': v:true })
        return jobId
    else
        return s:presenceJobId
    endif
endfunction

call s:connect()

augroup Presence
    autocmd!
    autocmd BufNewFile,BufRead,BufEnter * :UpdateRichPresence
augroup END
