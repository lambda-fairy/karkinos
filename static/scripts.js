document.addEventListener('DOMContentLoaded', () => {
    var searchForm = document.getElementById('search')
    if (!searchForm) return

    var searchBox = document.getElementById('q')
    var resultsElement = document.getElementById('results')

    if (!searchBox.value) searchBox.select()

    // Live search results!
    var searchTimer = null
    var oldValue = ''
    // Hack: the keypress event fires before `searchBox.value` is updated
    // We work around this by scheduling the handler after it settles
    searchBox.addEventListener('keypress', () => setTimeout(handleKeyPress, 0))
    var handleKeyPress = () => {
        // Don't reload results when the query hasn't changed
        if (searchBox.value === oldValue) return
        oldValue = searchBox.value
        // Debouncing: make the request 500 ms after user stops typing
        if (searchTimer) clearTimeout(searchTimer)
        resultsElement.innerHTML = ''
        searchTimer = setTimeout(() => {
            var r = new XMLHttpRequest()
            r.onreadystatechange = () => {
                if (r.readyState === XMLHttpRequest.DONE) {
                    resultsElement.innerHTML = r.status === 200 ?
                        r.responseText : 'Error: ' + r.status
                }
            }
            r.open('GET', '/search?raw=true&q=' + encodeURIComponent(searchBox.value))
            r.send()
        }, 500)
    }

    searchForm.addEventListener('submit', e => {
        e.preventDefault()
        if (searchBox.value) {
            var firstResult = resultsElement.querySelector('a:link')
            if (firstResult)
                firstResult.click()
        }
    })
})
