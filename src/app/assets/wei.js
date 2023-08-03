(() => {
  if (navigator.getEnvironmentIntegrity !== undefined) {
    document.title = "Access Blocked";
    document.body.innerHTML = `
        <style>
          a {
              text-decoration: none !important;
              color: #013C88 !important;
          }

          a:hover {
              color: #142E51 !important;
          }
      </style>

      <div style="
          background: #f54b39 !important;
          color: white !important;
          font-weight: bold !important;
          width: 100% !important;
          height: 100% !important;
          text-align: center !important;
          position: fixed !important;
          top: 0 !important;
          left: 0 !important;
          font-size: 1em !important;
          height: 100%;
          font-family: sans-serif !important;
          display: flex !important;
          align-items: center !important;
          z-index: 696969696969696969 !important;
      ">
          <p style="
              margin: 0 !important; 
              padding: 0 !important; 
              margin-top: 2.5% !important; 
              display: unset !important; 
              position: unset !important;
          ">
          We're sorry, but the browser you are currently using seems to support the <a href="https://github.com/RupertBenWiser/Web-Environment-Integrity">Web Environment Integrity API</a>. This user-hostile addition to Google-backed browsers <a href="https://yewtu.be/watch?v=0i0Ho-x7s_U">works to undermine the free and open internet</a>, and is consequently not supported. Please switch to the latest version of <a href="https://www.mozilla.org/en-US/firefox/new/">Firefox</a> or any other browser that still gives some value to the notion of user control and freedom to use this website. Thank you for your understanding.
          </p>
      </div>
    `;
  }
})();
